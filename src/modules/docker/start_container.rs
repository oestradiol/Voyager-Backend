use axum::http::StatusCode;
use bollard::container::StartContainerOptions;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

pub async fn start_container(container_name: String) -> Result<(), VoyagerError> {
  event!(
    Level::INFO,
    "Starting container with name: {}",
    container_name
  );

  DOCKER_RUNTIME
    .spawn_handled("modules::docker::start_container", async move {
      DOCKER
        .start_container(&container_name, None::<StartContainerOptions<String>>)
        .await
    })
    .await?
    .map_err(|e| VoyagerError::start_container(Box::new(e)))?;

  event!(Level::DEBUG, "Done starting container.");

  Ok(())
}

impl VoyagerError {
  fn start_container(e: Error) -> Self {
    let message = format!("Failed to start container! Error: {e}");
    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }
}
