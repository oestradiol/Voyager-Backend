use crate::modules::docker::{DOCKER, DOCKER_RUNTIME};
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::runtime_helpers::RuntimeSpawnHandled;
use crate::utils::Error;
use axum::http::StatusCode;
use bollard::container::RemoveContainerOptions;
use tracing::{event, Level};

pub async fn delete_container(container_name: String) -> Result<(), VoyagerError> {
  event!(Level::INFO, "Deleting container '{container_name}'");

  let options = Some(RemoveContainerOptions {
    v: true,
    force: false,
    link: false,
  });

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::delete_container", async move {
      DOCKER.remove_container(&container_name, options).await
    })
    .await?
    .map_err(|e| VoyagerError::delete_container(Box::new(e)))?;

  event!(Level::INFO, "Done deleting container");

  Ok(())
}

impl VoyagerError {
  pub fn delete_container(e: Error) -> Self {
    let message = format!("Failed to delete container! Error: {e}");
    event!(Level::ERROR, message);
    VoyagerError {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }
}
