// The Struct TransactionManager is used as a 'stack' and will have all the arguments used by each Command.
// They're also assigned in order, so there's no problem in using unwrap every time they're used.
// It was done like this in order to avoid multiple arguments in each Command + multiple Clones, effectively reducing the amount of code and making it also more efficient.
// Each case was thoroughly checked and should never panic. But any new changes should be carefully checked.
#![allow(clippy::unwrap_used)]

use std::fs;
use std::path::PathBuf;

use crate::business::repositories;
use crate::business::repositories::deployments::save;
use crate::business::services::SERVICES_RUNTIME;
use crate::configs::environment::{DEPLOYMENTS_DIR, HOST_IP};
use crate::modules::discord::send_deployment_message;
use crate::modules::{cloudflare, git};
use crate::types::model::deployment;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::{self};
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
  let final_branch: String;
  let mut log = format!("Creating deployment with host {host}, mode {mode}, repo_url {repo_url}");
  if let Some(branch) = branch.as_ref() {
    final_branch = branch.clone();
    log = format!("{log}, branch {branch}");
  } else {
    final_branch = "default".to_string();
    log = format!("{log}, branch default");
  }
  event!(Level::INFO, log);

  let future = async move {
    let container_name = host.replace('.', "-");

    let mut manager = TransactionManager {
      history: Vec::new(),
      next: Some(Box::new(GitClone)),

      repo_url: Some(repo_url),
      branch,
      final_branch: Some(final_branch),
      host: Some(host.clone()),
      mode: Some(mode),
      container_name: Some(container_name.clone()),
      
      dir_as_path: None,
      tar_path: None,
      container_id: None,
      // port: None,
      // internal_port: None,
      image_id: None,
      dns_record_id: None,
      final_id: None,
    };
    manager.start().await?;

    if let Some(db_id) = manager.final_id {
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

  dir_as_path: Option<PathBuf>,
  tar_path: Option<PathBuf>,
  repo_url: Option<String>,
  branch: Option<String>,
  final_branch: Option<String>,
  container_id: Option<String>,
  host: Option<String>,
  mode: Option<Mode>,
  // port: Option<u16>,
  // internal_port: Option<u16>,
  container_name: Option<String>,
  image_id: Option<String>,
  dns_record_id: Option<String>,

  final_id: Option<String>,
}

impl TransactionManager {
  async fn start(&mut self) -> Result<(), VoyagerError> {
    while let Some(mut command) = self.next.take() {
      let result = command.execute(self).await;
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
      command.undo(self).await;
    }
  }
}

#[async_trait]
trait Command: Sync + Send {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError>;
  async fn undo(&self, manager: &TransactionManager);
}

struct GitClone;
#[async_trait]
impl Command for GitClone {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    let directory = format!(
      "{}_{}_{}",
      manager.repo_url.as_ref().unwrap().replace('/', "_"),
      manager.final_branch.as_ref().unwrap(),
      Uuid::new_v4()
    );

    let base_dir = PathBuf::from(&*DEPLOYMENTS_DIR);
    if !base_dir.exists() {
      tokio::fs::create_dir_all(&base_dir).await.map_err(|e| VoyagerError::create_dir(Box::new(e)))?;
    }
    
    let dir_as_path = base_dir.join(&directory);
    git::clone(manager.repo_url.as_ref().unwrap(), manager.branch.take(), &dir_as_path)?;

    manager.dir_as_path = Some(dir_as_path);

    manager.next = Some(Box::new(CreateTar));

    Ok(())
  }

  async fn undo(&self, manager: &TransactionManager) {
    let dir_as_path = manager.dir_as_path.as_ref().unwrap();
    if dir_as_path.exists() {
      let _ = tokio::fs::remove_dir_all(dir_as_path)
        .await
        .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)));
    }
  }
}

struct CreateTar;
#[async_trait]
impl Command for CreateTar {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    let tar_path = tar::create(manager.dir_as_path.as_ref().unwrap()).await.map_err(|e| VoyagerError::create_tar(Box::new(e)))?;

    manager.tar_path = Some(tar_path);

    manager.next = Some(Box::new(CreateImage));

    Ok(())
  }

  async fn undo(&self, manager: &TransactionManager) {
    let tar_path = manager.tar_path.as_ref().unwrap();
    if tar_path.exists() {
      let _ = tokio::fs::remove_file(tar_path)
        .await
        .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)));
    }
  }
}

struct CreateImage;
#[async_trait]
impl Command for CreateImage {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    let dockerfile = manager.dir_as_path.as_ref().unwrap().join("Dockerfile");
    let dockerfile_contents =
      fs::read_to_string(&dockerfile).map_err(|e| VoyagerError::dockerfile_read(Box::new(e)))?;

    let internal_port = docker::find_internal_port(dockerfile_contents.as_str())?;
    let traefik_labels = utils::gen_traefik_labels(manager.container_name.as_ref().unwrap(), manager.host.as_ref().unwrap(), internal_port);

    let image_id = docker::build_image(manager.tar_path.as_ref().unwrap(), &traefik_labels, None).await?;

    // manager.internal_port = Some(internal_port);
    manager.image_id = Some(image_id);

    manager.next = Some(Box::new(CreateContainer));

    Ok(())
  }
  async fn undo(&self, manager: &TransactionManager) {
    let image_id = manager.image_id.clone().unwrap();
    let _ = docker::delete_image(image_id).await;
  }
}

struct CreateContainer;
#[async_trait]
impl Command for CreateContainer {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    tokio::fs::remove_dir_all(manager.dir_as_path.as_ref().unwrap())
      .await
      .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)))?;
    tokio::fs::remove_file(manager.tar_path.as_ref().unwrap()).await
      .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)))?;
    
    // let port = get_free_port()?;
    let container_id =
      docker::create_container(manager.container_name.clone().unwrap(),/* port, manager.internal_port.unwrap(), */manager.image_id.as_ref().unwrap()).await?;

    // manager.port = Some(port);
    manager.container_id = Some(container_id);

    manager.next = Some(Box::new(StartContainer));

    Ok(())
  }

  async fn undo(&self, manager: &TransactionManager) {
    let _ = docker::delete_container(manager.container_name.clone().unwrap()).await;
  }
}

struct StartContainer;
#[async_trait]
impl Command for StartContainer {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    docker::start_container(manager.container_name.clone().unwrap()).await?;

    manager.next = Some(Box::new(AddDNSRecord));

    Ok(())
  }

  async fn undo(&self, manager: &TransactionManager) {
    let _ = docker::stop_container(manager.container_name.clone().unwrap()).await;
  }
}

struct AddDNSRecord;
#[async_trait]
impl Command for AddDNSRecord {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    let dns_record_id = cloudflare::add_dns_record(manager.host.as_ref().unwrap(), &HOST_IP, manager.mode.as_ref().unwrap()).await?;

    manager.dns_record_id = Some(dns_record_id);

    manager.next = Some(Box::new(SaveDeployment));

    Ok(())
  }

  async fn undo(&self, manager: &TransactionManager) {
    let dns_record_id = manager.dns_record_id.as_ref().unwrap();
    let _ = cloudflare::delete_dns_record(dns_record_id).await;
  }
}

struct SaveDeployment;
#[async_trait]
impl Command for SaveDeployment {
  async fn execute(&mut self, manager: &mut TransactionManager) -> Result<(), VoyagerError> {
    let deployment = Deployment {
      _id: ObjectId::new(),
      container_id: manager.container_id.take().unwrap(),
      dns_record_id: manager.dns_record_id.clone().unwrap(),
      image_id: manager.image_id.clone().unwrap(),
      container_name: manager.container_name.clone().unwrap(),
      // port: manager.port.take().unwrap(),
      mode: manager.mode.take().unwrap(),
      host: manager.host.take().unwrap(),
      repo_url: manager.repo_url.take().unwrap(),
      branch: manager.final_branch.take().unwrap(),
    };
    
    let deployment_id = save(deployment).await?;

    manager.final_id = Some(deployment_id.to_string());
    manager.next = None;

    Ok(())
  }

  async fn undo(&self, manager: &TransactionManager) {
    let deployment_id = manager.final_id.as_ref().unwrap();
    let _ = repositories::deployments::delete(deployment_id).await;
  }
}

impl VoyagerError {
  fn null_db_id() -> Self {
    Self::new(
      "Failed to get DB Entity ID, it was null".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      None,
    )
  }

  fn create_dir(e: Error) -> Self {
    Self::new(
      "Failed to create deployments directory".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }

  fn delete_file_or_dir(e: Error) -> Self {
    Self::new(
      "Failed to delete directory or file for deployment".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }

  fn create_tar(e: Error) -> Self {
    Self::new(
      "Failed to create tar".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }

  fn dockerfile_read(e: Error) -> Self {
    Self::new(
      "Failed to read Dockerfile contents".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }
}