use axum::http::StatusCode;
use color_eyre::owo_colors::OwoColorize;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

pub async fn is_container_running(container_name: String) -> Result<bool, VoyagerError> {
  event!(
    Level::DEBUG,
    "Checking if container with name {container_name} is running"
  );

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::is_container_running", async move {
      DOCKER.inspect_container(&container_name, None).await
    })
    .await?
    .map_err(|e| VoyagerError::inspect_container(Box::new(e)))?;

  let result = result
    .state
    .map(|s| s.running)
    .and_then(|r| r)
    .ok_or_else(VoyagerError::empty_state)?;

  event!(Level::INFO, "Done checking if container is running");

  Ok(result)
}

impl VoyagerError {
  fn inspect_container(e: Error) -> Self {
    Self::new(
      "Failed to inspect container".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn empty_state() -> Self {
    Self::new(
      "State was None! Failed to get if container is running".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }
}
