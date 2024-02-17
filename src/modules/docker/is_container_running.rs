use color_eyre::owo_colors::OwoColorize;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  utils::runtime_helpers::RuntimeSpawnHandled,
  utils::Error,
};

async fn is_container_running(container_name: String) -> Option<bool> {
  event!(
    Level::DEBUG,
    "Checking if container with name {container_name} is running"
  );

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::is_container_running", async move {
      DOCKER.inspect_container(&container_name, None).await
    })
    .await
    .map(|r| {
      r.map_or_else(
        |e| {
          event!(Level::ERROR, "Failed to inspect container: {e}");
          None
        },
        |c| {
          c.state.map(|s| s.running).and_then(|r| r).map_or_else(
            || {
              event!(
                Level::ERROR,
                "State was None! Failed to get if container is running."
              );
              None
            },
            Some,
          )
        },
      )
    })
    .and_then(|i| i);

  event!(Level::INFO, "Done checking if container is running");

  result
}
