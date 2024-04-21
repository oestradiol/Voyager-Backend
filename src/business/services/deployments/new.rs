use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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
use crate::{business::repositories::deployments, modules::docker};
use async_trait::async_trait;
use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Bson;
use tokio::sync::Mutex;
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

    let git_clone = GitClone {
      manager: None,
      repo_url: repo_url.clone(),
      branch: branch.clone(),
      final_branch: final_branch.clone(),
      host: host.clone(),
      mode: mode.clone(),
      container_name: container_name.clone(),
      dir_as_path: None,
    };
    let manager = Arc::new(Mutex::new(TransactionManager::new(Box::new(git_clone))));
    let mut manager = manager.lock().await;
    manager.start().await?;

    if let Some(db_id) = manager.final_id.clone() {
      send_deployment_message(&db_id, &container_name, &host, &mode).await?;

      // TODO: notify user via email

      Ok(db_id)
    } else {
      Err(VoyagerError::null_db_id())
    }
  };

  let result = future.await;

  // let result = SERVICES_RUNTIME
  //   .spawn_handled("services::deployments::new", future)
  //   .await?;

  event!(Level::DEBUG, "Done creating deployment.");

  result
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

struct TransactionManager {
  history: Vec<Box<dyn Command>>,
  next: Option<Box<dyn Command>>,
  final_id: Option<String>,
}

impl TransactionManager {
  fn new(first: Box<dyn Command>) -> TransactionManager {
    TransactionManager {
      history: Vec::new(),
      next: Some(first),
      final_id: None,
    }
  }

  async fn start(&mut self) -> Result<(), VoyagerError> {
    while let Some(mut command) = self.next.take() {
      let result = command.execute().await;
      if let Err(e) = result {
        self.undo().await;
        return Err(e);
      }

      self.history.push(command);
    }
    Ok(())
  }

  async fn undo(&self) {
    for command in self.history.iter().rev() {
      command.undo().await;
    }
  }
}

#[async_trait(?Send)]
trait Command {
  async fn execute(&mut self) -> Result<(), VoyagerError>;
  async fn undo(&self);
}

struct GitClone {
  manager: Option<Arc<Mutex<TransactionManager>>>,
  repo_url: String,
  branch: Option<String>,
  final_branch: String,
  host: String,
  mode: Mode,
  container_name: String,
  dir_as_path: Option<PathBuf>,
}
#[async_trait(?Send)]
impl Command for GitClone {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
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

    let dir_as_path = atom.await?;

    self.dir_as_path = Some(dir_as_path.clone());
    if let Some(manager) = &self.manager {
      let create_tar = CreateTar {
        manager: manager.clone(),
        dir_as_path: dir_as_path.clone(),
        host: self.host.clone(),
        mode: self.mode.clone(),
        container_name: self.container_name.clone(),
        repo_url: self.repo_url.clone(),
        branch: self.final_branch.clone(),
        tar_path: None,
      };
      manager.lock().await.next = Some(Box::new(create_tar));
    }
    Ok(())
  }

  async fn undo(&self) {
    if let Some(dir_as_path) = &self.dir_as_path {
      if dir_as_path.exists() {
        tokio::fs::remove_dir_all(dir_as_path)
          .await
          .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)));
      }
    }
  }
}

struct CreateTar {
  manager: Arc<Mutex<TransactionManager>>,
  dir_as_path: PathBuf,
  host: String,
  mode: Mode,
  container_name: String,
  repo_url: String,
  branch: String,
  tar_path: Option<PathBuf>,
}
#[async_trait(?Send)]
impl Command for CreateTar {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
    let tar_path = tar::create(&self.dir_as_path).await.map_err(|e| VoyagerError::create_tar(Box::new(e)))?;
    self.tar_path = Some(tar_path.clone());
    let create_image = CreateImage {
      manager: self.manager.clone(),
      dir_as_path: self.dir_as_path.clone(),
      tar_path,
      host: self.host.clone(),
      mode: self.mode.clone(),
      container_name: self.container_name.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
      image_name: None,
    };
    self.manager.lock().await.next = Some(Box::new(create_image));
    Ok(())
  }

  async fn undo(&self) {
    if let Some(tar_path) = &self.tar_path {
      if tar_path.exists() {
        tokio::fs::remove_file(tar_path)
          .await
          .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)));
      }
    }
  }
}

struct CreateImage {
  manager: Arc<Mutex<TransactionManager>>,
  dir_as_path: PathBuf,
  tar_path: PathBuf,
  host: String,
  mode: Mode,
  container_name: String,
  repo_url: String,
  branch: String,
  image_name: Option<String>,
}
#[async_trait(?Send)]
impl Command for CreateImage {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
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
    let (image_name, container_name, internal_port) = atom.await?;
    self.image_name = Some(image_name.clone());
    let create_container = CreateContainer {
      manager: self.manager.clone(),
      dir_as_path: self.dir_as_path.clone(),
      tar_path: self.tar_path.clone(),
      container_name,
      internal_port,
      image_name,
      mode: self.mode.clone(),
      host: self.host.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
      container_id: None,
    };
    self.manager.lock().await.next = Some(Box::new(create_container));
    Ok(())
  }

  async fn undo(&self) {
    if let Some(image_name) = &self.image_name {
      docker::delete_image(image_name.clone()).await;
    }
  }
}

struct CreateContainer {
  manager: Arc<Mutex<TransactionManager>>,
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
#[async_trait(?Send)]
impl Command for CreateContainer {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
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
    let (container_id, port) = atom.await?;
    self.container_id = Some(container_id.clone());
    let start_container = StartContainer {
      manager: self.manager.clone(),
      container_id,
      image_name: self.image_name.clone(),
      container_name: self.container_name.clone(),
      port,
      mode: self.mode.clone(),
      host: self.host.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
    };
    self.manager.lock().await.next = Some(Box::new(start_container));
    Ok(())
  }

  async fn undo(&self) {
    if let Some(container_id) = &self.container_id {
      docker::delete_container(container_id.clone()).await;
    }
  }
}

struct StartContainer {
  manager: Arc<Mutex<TransactionManager>>,
  container_id: String,
  image_name: String,
  container_name: String,
  port: u16,
  mode: Mode,
  host: String,
  repo_url: String,
  branch: String,
}
#[async_trait(?Send)]
impl Command for StartContainer {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
    docker::start_container(self.container_name.clone()).await?;
    let add_dns_record = AddDNSRecord {
      manager: self.manager.clone(),
      container_id: self.container_id.clone(),
      image_name: self.image_name.clone(),
      container_name: self.container_name.clone(),
      port: self.port.clone(),
      mode: self.mode.clone(),
      host: self.host.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
      dns_record_id: None,
    };
    self.manager.lock().await.next = Some(Box::new(add_dns_record));
    Ok(())
  }

  async fn undo(&self) {
    docker::stop_container(self.container_id.clone()).await;
  }
}

struct AddDNSRecord {
  manager: Arc<Mutex<TransactionManager>>,
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
#[async_trait(?Send)]
impl Command for AddDNSRecord {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
    let dns_record_id = cloudflare::add_dns_record(&self.host, &HOST_IP, &self.mode).await?;
    self.dns_record_id = Some(dns_record_id.clone());
    let save_deployment = SaveDeployment {
      manager: self.manager.clone(),
      container_id: self.container_id.clone(),
      dns_record_id,
      image_name: self.image_name.clone(),
      container_name: self.container_name.clone(),
      port: self.port.clone(),
      mode: self.mode.clone(),
      host: self.host.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
      deployment_id: None,
    };
    self.manager.lock().await.next = Some(Box::new(save_deployment));
    Ok(())
  }

  async fn undo(&self) {
    if let Some(dns_record_id) = &self.dns_record_id {
      cloudflare::delete_dns_record(dns_record_id).await;
    }
  }
}

struct SaveDeployment {
  manager: Arc<Mutex<TransactionManager>>,
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
#[async_trait(?Send)]
impl Command for SaveDeployment {
  async fn execute(&mut self) -> Result<(), VoyagerError> {
    let deployment = Deployment {
      _id: ObjectId::new(),
      container_id: self.container_id.clone(),
      dns_record_id: self.dns_record_id.clone(),
      image_name: self.image_name.clone(),
      container_name: self.container_name.clone(),
      port: self.port.clone(),
      mode: self.mode.clone(),
      host: self.host.clone(),
      repo_url: self.repo_url.clone(),
      branch: self.branch.clone(),
    };
    
    let deployment_id = save(deployment).await?;
    let deployment_id = deployment_id.to_string();
    self.deployment_id = Some(deployment_id.clone());
    let mut manager = self.manager.lock().await;
    manager.final_id = Some(deployment_id);
    manager.next = None;
    Ok(())
  }

  async fn undo(&self) {
    if let Some(deployment_id) = &self.deployment_id {
      repositories::deployments::delete(deployment_id.clone()).await;
    }
  }
}