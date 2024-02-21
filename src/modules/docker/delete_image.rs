use crate::modules::docker::{DOCKER, DOCKER_RUNTIME};
use crate::utils::runtime_helpers::RuntimeSpawnHandled;
use crate::utils::Error;
use bollard::container::RemoveContainerOptions;
use bollard::image::RemoveImageOptions;
use tracing::{event, Level};

pub async fn delete_image(image_name: String) -> Option<()> {
  event!(Level::INFO, "Deleting image {image_name}");

  let options = Some(RemoveImageOptions {
    force: true,
    noprune: false,
  });

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::delete_image", async move {
      DOCKER.remove_image(&image_name, options, None).await
    })
    .await
    .map(|r| match r {
      Ok(_) => Some(()),
      Err(err) => {
        event!(Level::ERROR, "Failed to delete image: {}", err);
        None
      }
    })
    .and_then(|r| r);

  event!(Level::INFO, "Done deleting image");

  result
}
