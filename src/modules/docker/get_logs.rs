use bollard::container::LogsOptions;
use futures::StreamExt;
use tracing::{event, Level};

use crate::{
  modules::docker::{DOCKER, DOCKER_RUNTIME},
  utils::runtime_helpers::RuntimeSpawnHandled,
  utils::Error,
};

async fn get_logs(container_name: &str) -> Option<String> {
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
        .fold(String::new(), |acc, i| async {
          i.map_err(Error::from) // Converts a possible Bollard Error into our type of Error
            .map_or_else(
              |e| {
                event!(Level::ERROR, "Error trying to read logs: {:?}", e);
                acc
              },
              |d| d.to_string(),
            )
        }),
    )
    .await;

  event!(
    Level::DEBUG,
    "Done getting logs for container with name {container_name}"
  );

  logs
}
