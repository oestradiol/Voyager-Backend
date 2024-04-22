use axum::http::StatusCode;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

pub async fn stop_container(container_name: String) -> Result<(), VoyagerError> {
  event!(
    Level::INFO,
    "Stopping container with name: {}",
    container_name
  );

  DOCKER_RUNTIME
    .spawn_handled("modules::docker::stop_container", async move {
      DOCKER.stop_container(&container_name, None).await
    })
    .await?
    .map_err(|e| VoyagerError::stop_container(Box::new(e)))?;

  event!(Level::DEBUG, "Done stopping container.");

  Ok(())
}

impl VoyagerError {
  fn stop_container(e: Error) -> Self {
    Self::new(
      "Failed to stop container".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }
}
