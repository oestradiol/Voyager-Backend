use crate::{utils::runtime_helpers::RuntimeSpawnHandled, Error};
use bollard::image::BuildImageOptions;
use futures::StreamExt;
use std::{collections::HashMap, path::Path};

use super::{DOCKER, DOCKER_RUNTIME};
use tracing::{event, Level};

/// Builds docker image, then returns the image id
pub async fn build_image(
  dockerfile: &Path,
  labels: &Vec<(String, String)>,
  extra_hosts: Option<String>,
) -> Result<String, Error> {
  let dockerfile_str;
  match dockerfile.to_str() {
    Some(str) => dockerfile_str = str.to_string(),
    None => return Err(Error::from("Dockerfile path is not a valid unicode string")),
  }

  let build_image_options = BuildImageOptions {
    dockerfile: dockerfile_str.clone(),
    extrahosts: extra_hosts.map(|s| s),
    q: true,
    memory: Some(700 * 1024 * 1024),  // 700MiB
    memswap: Some(500 * 1024 * 1024), // 500MiB
    labels: labels.iter().fold(HashMap::new(), |mut acc, p| {
      acc.insert(p.0.clone(), p.1.clone());
      acc
    }),
    ..Default::default()
  };

  event!(
    Level::INFO,
    "Building docker image with tags: {:?}, Dockerfile: {:?}",
    &build_image_options.labels,
    &build_image_options.dockerfile
  );

  event!(Level::DEBUG, "Done building docker image.");

  let result = _build_image(build_image_options).await;

  match result {
    Some(image_id) => Ok(image_id),
    None => Err(Error::from(format!(
      "Failed to build docker image for dockerfile {dockerfile_str}"
    ))),
  }
}

async fn _build_image(options: BuildImageOptions<String>) -> Option<String> {
  let future = async move {
    let build_stream = DOCKER.build_image(options, None, None);

    let img_id = build_stream
      .fold(String::new(), |acc, i| async {
        i.map_err(Error::from) // Converts a possible Bollard Error into our type of Error
          .map(|r| r.aux) // Extracts the aux field from the response, it is an Option<ImageId>
          .map(|i| i.map(|i| i.id)) // Extracts the id field from the ImageId, it is also an Option<String>
          .map(|i| i.and_then(|i| i)) // Flattens the Option<Option<String>> into an Option<String>
          .and_then(|i| {
            i.ok_or_else(|| Error::from("Error trying to build docker image. Stream was empty."))
          }) // Converts the Option<String> into a Result<String, Error>
          .map_or_else(
            |e| {
              event!(Level::ERROR, "Error trying to build docker image: {:?}", e);
              acc
            },
            |d| d,
          ) // Logs the error then returns the previous value of acc or simply returns the Image Id, phew!
      })
      .await;

    if !img_id.is_empty() {
      event!(
        Level::DEBUG,
        "Docker image built successfully! Id: {}",
        img_id
      );
      return Some(img_id);
    }

    event!(Level::INFO, "Image Id is None!");
    None
  };

  DOCKER_RUNTIME
    .spawn_handled("modules::docker::build_image", future)
    .await
    .and_then(|r| r)
}
