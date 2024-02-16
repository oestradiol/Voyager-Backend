use bollard::container::StartContainerOptions;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  utils::runtime_helpers::RuntimeSpawnHandled,
};

async fn start_container(container_name: String) -> bool {
  event!(
    Level::INFO,
    "Starting container with name: {}",
    container_name
  );

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::start_container", async move {
      DOCKER
        .start_container(&container_name, None::<StartContainerOptions<String>>)
        .await
    })
    .await
    .map_or_else(
      || false,
      |r| {
        r.map_or_else(
          |e| {
            event!(Level::ERROR, "Failed to start container: {e}");
            false
          },
          |()| true,
        )
      },
    );

  event!(Level::DEBUG, "Done starting container.");

  result
}
