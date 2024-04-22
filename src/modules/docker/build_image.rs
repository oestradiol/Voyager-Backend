use crate::{
  configs::environment::DEVELOPMENT, types::other::voyager_error::VoyagerError, utils::{runtime_helpers::RuntimeSpawnHandled, Error}
};
use axum::http::StatusCode;
use bollard::image::BuildImageOptions;
use futures::StreamExt;
use std::{collections::HashMap, path::Path};

use super::{DOCKER, DOCKER_RUNTIME};
use tracing::{event, Level};

pub async fn build_image(
  tar: &Path,
  labels: &[(String, String)],
  extra_hosts: Option<String>,
) -> Result<String, VoyagerError> {
  let options = BuildImageOptions {
    dockerfile: "Dockerfile".to_string(),
    extrahosts: extra_hosts,
    q: false,
    forcerm: true,
    memory: Some(2048 * 1024 * 1024),  // 2GiB
    memswap: Some(2049 * 1024 * 1024), // 2GiB
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

  let contents = tokio::fs::read(tar).await
    .map_err(|e| VoyagerError::file_read_error(Box::new(e)))?;
  let result = DOCKER_RUNTIME
    .spawn_handled("modules::docker::build_image", async move {
      DOCKER
        .build_image(options, None, Some(contents.into()))
        .fold(String::new(), |acc, i| async move {
          i.map(|build_info| {
              if !&*DEVELOPMENT {
                event!(Level::INFO, "Response: {:?}", build_info.stream.unwrap_or_default());
              }
              build_info.aux.map(|i| i.id).and_then(|i| i)
            })
            .map_or_else(|e| {
              VoyagerError::intermediate_build_image(Box::new(e));
              acc.clone()
            }, |i| i.unwrap_or_else(|| acc.clone())) 
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
  fn file_read_error(e: Error) -> Self {
    Self::new(
      "Failed to read tar file!".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }

  fn intermediate_build_image(e: Error) -> Self {
    Self::new(
      "Error during Docker Image Build intermediate steps".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      true,
      Some(e),
    )
  }

  fn build_image() -> Self {
    Self::new(
      "Failed to build image!".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      None,
    )
  }
}
