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

  DOCKER_RUNTIME
    .spawn_handled("modules::docker::delete_container", async move {
      DOCKER.remove_container(&container_name, options).await
    })
    .await?
    .map_err(|e| VoyagerError::delete_container(Box::new(e)))?;

  event!(Level::DEBUG, "Done deleting container");

  Ok(())
}

impl VoyagerError {
  fn delete_container(e: Error) -> Self {
    Self::new(
      "Failed to delete container".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
