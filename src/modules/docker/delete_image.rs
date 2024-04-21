use crate::modules::docker::{DOCKER, DOCKER_RUNTIME};
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::runtime_helpers::RuntimeSpawnHandled;
use crate::utils::Error;
use axum::http::StatusCode;
use bollard::image::RemoveImageOptions;
use tracing::{event, Level};

pub async fn delete_image(image_name: String) -> Result<(), VoyagerError> {
  event!(Level::INFO, "Deleting image {image_name}");

  let options = Some(RemoveImageOptions {
    force: true,
    noprune: false,
  });

  DOCKER_RUNTIME
    .spawn_handled("modules::docker::delete_image", async move {
      DOCKER.remove_image(&image_name, options, None).await
    })
    .await?
    .map_err(|e| VoyagerError::remove_image(Box::new(e)))?;

  event!(Level::DEBUG, "Done deleting image");

  Ok(())
}

impl VoyagerError {
  fn remove_image(e: Error) -> Self {
    Self::new(
      "Failed to remove image".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
