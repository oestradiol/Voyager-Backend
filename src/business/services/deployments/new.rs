use std::fs;
use std::path::Path;

use crate::business::repositories::deployments::save;
use crate::configs::environment::HOST_IP;
use crate::modules::discord::send_deployment_message;
use crate::modules::{cloudflare, traefik};
use crate::types::model::deployment;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::get_free_port;
use crate::{business::repositories::deployments, modules::docker};
use axum::http::StatusCode;
use mongodb::bson::Bson;
use tracing::{event, Level};
use uuid::Uuid;

use crate::{
  types::model::deployment::{Deployment, Mode},
  utils::Error,
};

// Result<Result<T, Err>, Exc>
// Result<T, Result<Err, Exc>>

pub async fn new(
  dockerfile: &Path,
  host: &str,
  mode: &deployment::Mode,
  directory: &str,
  repo_url: &str,
  branch: &str,
) -> Result<Bson, VoyagerError> {
  event!(Level::INFO, "Creating deployment with host {host}, Dockerfile {:?}, mode {mode}, directory {directory}, repo_url {repo_url}, branch {branch}", dockerfile);

  let dns_record_id = cloudflare::add_dns_record(host, &HOST_IP, mode).await?;

  let dockerfile_contents =
    fs::read_to_string(dockerfile).map_err(|e| VoyagerError::dockerfile_read(Box::new(e)))?;

  let internal_port = docker::find_internal_port(dockerfile_contents.as_str())?;
  let free_port = get_free_port()?;

  let name = host.replace('.', "");
  let traefik_labels = traefik::gen_traefik_labels(&name, host, internal_port);

  let image_name = docker::build_image(dockerfile, &traefik_labels, None).await?;

  let container_id =
    docker::create_container(name.clone(), free_port, internal_port, image_name.as_str()).await?;

  let deployment = Deployment {
    container_id,
    dns_record_id,
    image_name,
    container_name: name.clone(),
    internal_port,
    mode: mode.to_owned(),
    host: host.to_string(),
    repo_url: repo_url.to_string(),
    branch: branch.to_string(),
  };

  let db_id = save(deployment).await?;

  send_deployment_message(db_id.to_string().as_str(), &name, host, mode).await?;

  // TODO: notify user via email

  Ok(db_id)
}

impl VoyagerError {
  fn dockerfile_read(e: Error) -> Self {
    let message = format!("Failed to read Dockerfile contents: {e}");
    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }
}
