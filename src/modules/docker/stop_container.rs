use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  utils::runtime_helpers::RuntimeSpawnHandled,
};

async fn stop_container(container_name: String) -> Option<()> {
  event!(
    Level::INFO,
    "Stopping container with name: {}",
    container_name
  );

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::stop_container", async move {
      DOCKER.stop_container(&container_name, None).await
    })
    .await
    .map_or_else(
      || None,
      |r| {
        r.map_or_else(
          |e| {
            event!(Level::ERROR, "Failed to stop container: {e}");
            None
          },
          |()| Some(()),
        )
      },
    );

  event!(Level::DEBUG, "Done stopping container.");

  result
}
