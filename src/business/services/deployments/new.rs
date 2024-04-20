use std::fs;
use std::path::{Path, PathBuf};

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
use axum::http::StatusCode;
use mongodb::bson::Bson;
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
) -> Result<Bson, VoyagerError> {
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
    let directory = format!(
      "{}_{final_branch}_{}",
      repo_url.replace('/', "_"),
      Uuid::new_v4()
    );

    let base_dir = PathBuf::from(&*DEPLOYMENTS_DIR);
    if !base_dir.exists() {
      tokio::fs::create_dir_all(&base_dir).await.map_err(|e| VoyagerError::create_dir(Box::new(e)))?;
    }
    let dir_as_path = base_dir.join(&directory);
    git::clone(&repo_url, branch, &dir_as_path)?;
    let tar = tar::create(&dir_as_path).await.map_err(|e| VoyagerError::create_tar(Box::new(e)))?;

    let dockerfile = dir_as_path.join("Dockerfile");
    let dockerfile_contents =
      fs::read_to_string(&dockerfile).map_err(|e| VoyagerError::dockerfile_read(Box::new(e)))?;

    let internal_port = docker::find_internal_port(dockerfile_contents.as_str())?;
    let free_port = get_free_port()?;

    let name = host.replace('.', "-");
    let traefik_labels = traefik::gen_traefik_labels(&name, &host, internal_port);

    let image_name = docker::build_image(&tar, &traefik_labels, None).await?;

    tokio::fs::remove_dir_all(dir_as_path)
      .await
      .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)))?;
    tokio::fs::remove_file(tar).await
      .map_err(|e| VoyagerError::delete_file_or_dir(Box::new(e)))?;

    let container_id =
      docker::create_container(name.clone(), free_port, internal_port, image_name.as_str()).await?;

    let dns_record_id = cloudflare::add_dns_record(&host, &HOST_IP, &mode).await?;

    let deployment = Deployment {
      container_id,
      dns_record_id,
      image_name,
      container_name: name.clone(),
      internal_port,
      mode,
      host: host.to_string(),
      directory,
      repo_url: repo_url.to_string(),
      branch: final_branch,
    };

    let db_id = save(deployment).await?;

    send_deployment_message(db_id.to_string().as_str(), &name, &host, &mode).await?;

    // TODO: notify user via email

    Ok(db_id)
  };

  let result = SERVICES_RUNTIME
    .spawn_handled("services::deployments::new", future)
    .await?;

  event!(Level::DEBUG, "Done creating deployment.");

  result
}

impl VoyagerError {
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
