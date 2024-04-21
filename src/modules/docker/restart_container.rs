use axum::http::StatusCode;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

#[allow(unused)]
pub async fn restart_container(container_name: String) -> Result<(), VoyagerError> {
  event!(
    Level::INFO,
    "Restarting container with name: {}",
    container_name
  );

  DOCKER_RUNTIME
    .spawn_handled("modules::docker::restart_container", async move {
      DOCKER.restart_container(&container_name, None).await
    })
    .await?
    .map_err(|e| VoyagerError::restart_container(Box::new(e)))?;

  event!(Level::DEBUG, "Done restarting container.");

  Ok(())
}

impl VoyagerError {
  fn restart_container(e: Error) -> Self {
    Self::new(
      "Failed to restart container".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
