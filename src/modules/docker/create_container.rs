use std::collections::HashMap;

use crate::utils::runtime_helpers::RuntimeSpawnHandled;
use bollard::{
  container::{Config, CreateContainerOptions},
  service::{HostConfig, PortBinding},
};
use tracing::{event, Level};

use super::{DOCKER, DOCKER_RUNTIME};

/// Creates container and returns id if successful
pub async fn create_container(
  name: String,
  port: u16,
  internal_port: u16,
  docker_image: String,
) -> Option<String> {
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
    image: Some(docker_image),
    host_config: Some(host_config),
    ..Default::default()
  };

  let result = DOCKER_RUNTIME
    .spawn_handled(
      "modules::docker::create_container",
      DOCKER.create_container(options, config),
    )
    .await
    .map(|res| match res {
      Ok(res) => Some(res.id),
      Err(err) => {
        event!(
          Level::ERROR,
          "Failed to create Docker container! Error: {}",
          err
        );
        None
      }
    })
    .and_then(|res| res);

  event!(Level::DEBUG, "Done creating new container.");

  result
}
