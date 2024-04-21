use std::collections::HashMap;

use crate::{
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use bollard::{
  container::{Config, CreateContainerOptions},
  service::{HostConfig, PortBinding},
};
use tracing::{event, Level};

use super::{DOCKER, DOCKER_RUNTIME};

pub async fn create_container(
  name: String,
  port: u16,
  internal_port: u16,
  docker_image: &str,
) -> Result<String, VoyagerError> {
  event!(
    Level::INFO,
    "Creating a new container {name} at port {port}. Docker Image: {docker_image}"
  );

  let options = Some(CreateContainerOptions {
    name,
    platform: Some("linux/amd64".to_string()),
  });

  let host_config = HostConfig {
    port_bindings: Some(HashMap::from([(
      internal_port.to_string(),
      Some(vec![PortBinding {
        host_ip: Some("127.0.0.1".to_string()),
        host_port: Some(port.to_string()),
      }]),
    )])),
    ..Default::default()
  };

  let config = Config {
    image: Some(docker_image.to_string()),
    host_config: Some(host_config),
    ..Default::default()
  };

  let result = DOCKER_RUNTIME
    .spawn_handled(
      "modules::docker::create_container",
      DOCKER.create_container(options, config),
    )
    .await?
    .map_or_else(
      |e| Err(VoyagerError::create_container(Box::new(e))),
      |res| Ok(res.id),
    )?;

  event!(Level::DEBUG, "Done creating new container.");

  Ok(result)
}

impl VoyagerError {
  fn create_container(e: Error) -> Self {
    Self::new(
      "Failed to create container".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
