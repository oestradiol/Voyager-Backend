use std::fs;
use std::path::PathBuf;

use crate::business::repositories;
use crate::business::repositories::deployments::save;
use crate::business::services::SERVICES_RUNTIME;
use crate::configs::environment::{DEPLOYMENTS_DIR, HOST_IP};
use crate::modules::discord::send_deployment_message;
use crate::modules::{cloudflare, git, traefik};
use crate::types::model::deployment;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::get_free_port;
use crate::utils::runtime_helpers::RuntimeSpawnHandled;
use crate::modules::docker;
use async_trait::async_trait;
use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
  modules::tar,
  types::model::deployment::{Deployment, Mode},
  utils::Error,
};

pub async fn new(
  host: String,
  mode: deployment::Mode,
  repo_url: String,
  branch: Option<String>,
) -> Result<String, VoyagerError> {
  let mut final_branch = "default".to_string();

  let mut log = format!("Creating deployment with host {host}, mode {mode}, repo_url {repo_url}");
  if let Some(branch) = branch.clone() {
    final_branch.clone_from(&branch);
    log = format!("{log}, branch {branch}");
  } else {
    log = format!("{log}, branch default");
  }
  event!(Level::INFO, log);

  let future = async move {
    let container_name = host.replace('.', "-");

    let mut manager = TransactionManager::new();
    let git_clone = GitClone {
      repo_url: repo_url.clone(),
      branch: branch.clone(),
      final_branch: final_branch.clone(),
      host: host.clone(),
      mode,
      container_name: container_name.clone(),
      dir_as_path: None,
    };
    manager.add_first(Box::new(git_clone));
    manager = manager.start().await?;

    if let Some(db_id) = manager.final_id.clone() {
      send_deployment_message(&db_id, &container_name, &host, &mode).await?;

      // TODO: notify user via email

      Ok(db_id)
    } else {
      Err(VoyagerError::null_db_id())
    }
  };

  let result = SERVICES_RUNTIME
    .spawn_handled("services::deployments::new", future)
    .await?;

  event!(Level::DEBUG, "Done creating deployment.");

  result
}

struct TransactionManager {
  history: Vec<Box<dyn Command>>,
  next: Option<Box<dyn Command>>,
  final_id: Option<String>,
}

impl TransactionManager {
  fn new() -> Self {
    Self {
      history: Vec::new(),
      next: None,
      final_id: None,
    }
  }

  fn add_first(&mut self, command: Box<dyn Command>) {
    self.next = Some(command);
  }

  async fn start(self) -> Result<Self, VoyagerError> {
    let mut manager = self;
    while let Some(mut command) = manager.next.take() {
      let (mut inner_manager, result) = command.execute(manager).await;
      if let Err(e) = result {
        inner_manager.undo().await;
        return Err(e);
      }

      inner_manager.history.push(command);
      manager = inner_manager;
    }
    Ok(manager)
  }

  async fn undo(&self) {
    for command in self.history.iter().rev() {
      command.undo().await;
    }
  }
}

#[async_trait]
trait Command: Sync + Send {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>);
  async fn undo(&self);
}

struct GitClone {
  repo_url: String,
  branch: Option<String>,
  final_branch: String,
  host: String,
  mode: Mode,
  container_name: String,
  dir_as_path: Option<PathBuf>,
}
#[async_trait]
impl Command for GitClone {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>) {
    let atom = async {
      let directory = format!(
        "{}_{}_{}",
        self.repo_url.replace('/', "_"),
        self.final_branch,
        Uuid::new_v4()
      );
  
      let base_dir = PathBuf::from(&*DEPLOYMENTS_DIR);
      if !base_dir.exists() {
        tokio::fs::create_dir_all(&base_dir).await.map_err(|e| VoyagerError::create_dir(Box::new(e)))?;
      }
      
      let dir_as_path = base_dir.join(&directory);
      git::clone(&self.repo_url, self.branch.take(), &dir_as_path)?;
      Ok(dir_as_path)
    };

    match atom.await {
      Err(e) => (manager, Err(e)),
      Ok(dir_as_path) => {
        self.dir_as_path = Some(dir_as_path.clone());
        let create_tar = CreateTar {
          dir_as_path,
          host: self.host.clone(),
          mode: self.mode,
          container_name: self.container_name.clone(),
          repo_url: self.repo_url.clone(),
          branch: self.final_branch.clone(),
          tar_path: None,
        };
        manager.next = Some(Box::new(create_tar));
        (manager, Ok(()))
      }
    }
  }

  async fn undo(&self) {
    if let Some(dir_as_path) = &self.dir_as_path {
      if dir_as_path.exists() {
        let _ = tokio::fs::remove_dir_all(dir_as_path)
          .await
          .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)));
      }
    }
  }
}

struct CreateTar {
  dir_as_path: PathBuf,
  host: String,
  mode: Mode,
  container_name: String,
  repo_url: String,
  branch: String,
  tar_path: Option<PathBuf>,
}
#[async_trait]
impl Command for CreateTar {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>) {
    let result = tar::create(&self.dir_as_path).await.map_err(|e| VoyagerError::create_tar(Box::new(e)));
    match result {
      Err(e) => (manager, Err(e)),
      Ok(tar_path) => {
        self.tar_path = Some(tar_path.clone());
        let create_image = CreateImage {
          dir_as_path: self.dir_as_path.clone(),
          tar_path,
          host: self.host.clone(),
          mode: self.mode,
          container_name: self.container_name.clone(),
          repo_url: self.repo_url.clone(),
          branch: self.branch.clone(),
          image_name: None,
        };
        manager.next = Some(Box::new(create_image));
        (manager, Ok(()))
      }
    }
  }

  async fn undo(&self) {
    if let Some(tar_path) = &self.tar_path {
      if tar_path.exists() {
        let _ = tokio::fs::remove_file(tar_path)
          .await
          .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)));
      }
    }
  }
}

struct CreateImage {
  dir_as_path: PathBuf,
  tar_path: PathBuf,
  host: String,
  mode: Mode,
  container_name: String,
  repo_url: String,
  branch: String,
  image_name: Option<String>,
}
#[async_trait]
impl Command for CreateImage {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>) {
    let atom = async {
      let dockerfile = self.dir_as_path.join("Dockerfile");
      let dockerfile_contents =
        fs::read_to_string(&dockerfile).map_err(|e| VoyagerError::dockerfile_read(Box::new(e)))?;
  
      let internal_port = docker::find_internal_port(dockerfile_contents.as_str())?;

      let container_name = self.container_name.clone();
      let traefik_labels = traefik::gen_traefik_labels(&container_name, &self.host, internal_port);
  
      let image_name = docker::build_image(&self.tar_path, &traefik_labels, None).await?;
      Ok((image_name, container_name, internal_port))
    };
    match atom.await {
      Err(e) => (manager, Err(e)),
      Ok((image_name, container_name, internal_port)) => {
        self.image_name = Some(image_name.clone());
        let create_container = CreateContainer {
          dir_as_path: self.dir_as_path.clone(),
          tar_path: self.tar_path.clone(),
          container_name,
          internal_port,
          image_name,
          mode: self.mode,
          host: self.host.clone(),
          repo_url: self.repo_url.clone(),
          branch: self.branch.clone(),
          container_id: None,
        };
        manager.next = Some(Box::new(create_container));
        (manager, Ok(()))
      }
    }
  }
  async fn undo(&self) {
    if let Some(image_name) = &self.image_name {
      let _ = docker::delete_image(image_name.clone()).await;
    }
  }
}

struct CreateContainer {
  dir_as_path: PathBuf,
  tar_path: PathBuf,
  container_name: String,
  internal_port: u16,
  image_name: String,
  mode: Mode,
  host: String,
  repo_url: String,
  branch: String,
  container_id: Option<String>,
}
#[async_trait]
impl Command for CreateContainer {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>){
    let atom = async {
      tokio::fs::remove_dir_all(&self.dir_as_path)
        .await
        .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)))?;
      tokio::fs::remove_file(&self.tar_path).await
        .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)))?;
      
      let free_port = get_free_port()?;
      let container_id =
        docker::create_container(self.container_name.clone(), free_port, self.internal_port, &self.image_name).await?;
      Ok((container_id, free_port))
    };
    match atom.await {
      Err(e) => (manager, Err(e)),
      Ok((container_id, port)) => {
        self.container_id = Some(container_id.clone());
        let start_container = StartContainer {
          container_id,
          image_name: self.image_name.clone(),
          container_name: self.container_name.clone(),
          port,
          mode: self.mode,
          host: self.host.clone(),
          repo_url: self.repo_url.clone(),
          branch: self.branch.clone(),
        };
        manager.next = Some(Box::new(start_container));
        (manager, Ok(()))
      }
    }
  }

  async fn undo(&self) {
    let _ = docker::delete_container(self.container_name.clone()).await;
  }
}

struct StartContainer {
  container_id: String,
  image_name: String,
  container_name: String,
  port: u16,
  mode: Mode,
  host: String,
  repo_url: String,
  branch: String,
}
#[async_trait]
impl Command for StartContainer {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>) {
    if let Err(e) = docker::start_container(self.container_name.clone()).await { (manager, Err(e)) } else { 
      let add_dns_record = AddDNSRecord {
        container_id: self.container_id.clone(),
        image_name: self.image_name.clone(),
        container_name: self.container_name.clone(),
        port: self.port,
        mode: self.mode,
        host: self.host.clone(),
        repo_url: self.repo_url.clone(),
        branch: self.branch.clone(),
        dns_record_id: None,
      };
      manager.next = Some(Box::new(add_dns_record));
      (manager, Ok(()))
    }
  }

  async fn undo(&self) {
    let _ = docker::stop_container(self.container_name.clone()).await;
  }
}

struct AddDNSRecord {
  container_id: String,
  image_name: String,
  container_name: String,
  port: u16,
  mode: Mode,
  host: String,
  repo_url: String,
  branch: String,
  dns_record_id: Option<String>,
}
#[async_trait]
impl Command for AddDNSRecord {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>) {
    let result = cloudflare::add_dns_record(&self.host, &HOST_IP, &self.mode).await;
    match result {
      Err(e) => (manager, Err(e)),
      Ok(dns_record_id) => {
        self.dns_record_id = Some(dns_record_id.clone());
        let save_deployment = SaveDeployment {
          container_id: self.container_id.clone(),
          dns_record_id,
          image_name: self.image_name.clone(),
          container_name: self.container_name.clone(),
          port: self.port,
          mode: self.mode,
          host: self.host.clone(),
          repo_url: self.repo_url.clone(),
          branch: self.branch.clone(),
          deployment_id: None,
        };
        manager.next = Some(Box::new(save_deployment));
        (manager, Ok(()))
      }
    }
  }

  async fn undo(&self) {
    if let Some(dns_record_id) = &self.dns_record_id {
      let _ = cloudflare::delete_dns_record(dns_record_id).await;
    }
  }
}

struct SaveDeployment {
  container_id: String,
  dns_record_id: String,
  image_name: String,
  container_name: String,
  port: u16,
  mode: Mode,
  host: String,
  repo_url: String,
  branch: String,
  deployment_id: Option<String>,
}
#[async_trait]
impl Command for SaveDeployment {
  async fn execute(&mut self, mut manager: TransactionManager) -> (TransactionManager, Result<(), VoyagerError>) {
    let deployment = Deployment {
      _id: ObjectId::new(),
      container_id: self.container_id.clone(),
      dns_record_id: self.dns_record_id.clone(),
      image_name: self.image_name.clone(),
      container_name: self.container_name.clone(),
      port: self.port,
      mode: self.mode,
      host: self.host.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
    };
    
    let result = save(deployment).await;

    match result {
      Err(e) => (manager, Err(e)),
      Ok(deployment_id) => {
        let deployment_id = deployment_id.to_string();
        self.deployment_id = Some(deployment_id.clone());
        manager.final_id = Some(deployment_id);
        manager.next = None;
        (manager, Ok(()))
      }
    }
  }

  async fn undo(&self) {
    if let Some(deployment_id) = &self.deployment_id {
      let _ = repositories::deployments::delete(deployment_id.clone()).await;
    }
  }
}

impl VoyagerError {
  fn null_db_id() -> Self {
    Self::new(
      "DB Entity ID was null".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }

  fn create_dir(e: Error) -> Self {
    Self::new(
      "Failed to create deployments directory".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn delete_file_or_dir(e: Error) -> Self {
    Self::new(
      "Failed to delete directory or file for deployment".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn create_tar(e: Error) -> Self {
    Self::new(
      "Failed to create tar".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn dockerfile_read(e: Error) -> Self {
    Self::new(
      "Failed to read Dockerfile contents".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}