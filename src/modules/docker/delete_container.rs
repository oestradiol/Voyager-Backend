use crate::modules::docker::{DOCKER, DOCKER_RUNTIME};
use crate::utils::runtime_helpers::RuntimeSpawnHandled;
use bollard::container::RemoveContainerOptions;
use tracing::{event, Level};

pub async fn delete_container(container_name: String) -> Option<()> {
  event!(Level::INFO, "Deleting container '{container_name}'");

  let options = Some(RemoveContainerOptions {
    v: true,
    force: false,
    link: false,
  });

  if let Some(res) = DOCKER_RUNTIME
    .spawn_handled("modules::docker::delete_container", async move {
      DOCKER.remove_container(&container_name, options).await
    })
    .await
  {
    match res {
      Ok(()) => (),
      Err(err) => {
        event!(
          Level::ERROR,
          "Failed to delete Docker container! Error: {}",
          err
        );
        return None;
      }
    }
  }

  event!(Level::INFO, "Done deleting container");
  Some(())
}
