use bollard::container::LogsOptions;
use futures::StreamExt;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

pub async fn get_logs(container_name: &str) -> Result<Vec<String>, VoyagerError> {
  event!(
    Level::INFO,
    "Getting logs for container with name {}",
    container_name
  );

  let options = LogsOptions::<String> {
    stdout: true,
    stderr: true,
    ..Default::default()
  };

  let logs = DOCKER_RUNTIME
    .spawn_handled(
      "modules::docker::get_logs",
      DOCKER
        .logs(container_name, Some(options))
        .fold(Vec::new(), |mut acc, i| async {
          match i {
            Ok(d) => acc.push(d.to_string()),
            Err(e) => event!(Level::ERROR, "Error trying to read logs: {:?}", e),
          }

          acc
        }),
    )
    .await?;

  event!(
    Level::DEBUG,
    "Done getting logs for container with name {container_name}"
  );

  Ok(logs)
}
