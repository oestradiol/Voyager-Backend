use bollard::image::BuildImageOptions;
use std::collections::HashMap;
use super::client::DOCKER;
use tracing::{event, Level};

/// Builds docker image, then returns the image id
pub async fn build_docker_image(
  dockerfile: String,
  labels: Vec<(String, String)>,
  extra_hosts: Option<String>,
) -> Option<String> {
  let build_image_options = BuildImageOptions {
    dockerfile: dockerfile,
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

  return _build_docker_image(build_image_options).await;
}

async fn _build_docker_image(options: BuildImageOptions<String>) -> Option<String> {
  todo!();

  let mut build_stream = DOCKER.build_image(options, None, None);

  let mut last: Option<String> = None;

  last
}
