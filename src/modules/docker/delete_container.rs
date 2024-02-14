use bollard::container::RemoveContainerOptions;
use tracing::{event, Level};
use crate::modules::docker::{DOCKER, DOCKER_RUNTIME};
use crate::utils::runtime_helpers::RuntimeSpawnHandled;

pub async fn delete_container(container_name: String) {
  event!(Level::INFO, "Deleting container '{container_name}'");

  let options = Some(RemoveContainerOptions {
    v: true,
    force: false,
    link: false,
  });


  DOCKER_RUNTIME.spawn_handled("modules::docker::delete_container", async move {
    DOCKER.remove_container(&container_name, options).await
  }).await
    .map(|res| {
      match res {
        Ok(_) => (),
        Err(err) => event!(Level::ERROR, "Failed to delete Docker container! Error: {}", err)
      }
    });
}
