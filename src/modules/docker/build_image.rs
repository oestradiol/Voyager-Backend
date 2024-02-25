use crate::{
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use bollard::image::BuildImageOptions;
use futures::StreamExt;
use std::{collections::HashMap, path::Path};

use super::{DOCKER, DOCKER_RUNTIME};
use tracing::{event, Level};

/// Builds docker image, then returns the image id
pub async fn build_image(
  dockerfile: &Path,
  labels: &[(String, String)],
  extra_hosts: Option<String>,
) -> Result<String, VoyagerError> {
  let dockerfile_str = dockerfile
    .to_str()
    .ok_or_else(|| VoyagerError::path_to_string())?
    .to_string();

  let options = BuildImageOptions {
    dockerfile: dockerfile_str.clone(),
    extrahosts: extra_hosts,
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
    &options.labels,
    &options.dockerfile
  );

  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::build_image", async move {
      DOCKER
        .build_image(options, None, None)
        .fold(String::new(), |acc, i| async {
          i.map_err(Error::from) // Converts a possible Bollard Error into our type of Error
            .map(|i| i.aux.map(|i| i.id))
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
        .await
    })
    .await?;

  let result = (if result.is_empty() {
    Err(VoyagerError::build_image())
  } else {
    Ok(result)
  })?;

  event!(Level::DEBUG, "Done building docker image.");

  Ok(result)
}

impl VoyagerError {
  pub fn build_image() -> Self {
    let message = format!("Failed to build image! Image Id was empty.");
    event!(Level::ERROR, message);
    VoyagerError {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: None,
    }
  }

  pub fn path_to_string() -> Self {
    let message = format!("Failed to convert Path to String!");
    event!(Level::ERROR, message);
    VoyagerError {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: None,
    }
  }
}
