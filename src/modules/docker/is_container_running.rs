use bollard::container::InspectContainerOptions;
use tracing::{event, Level};


use super::DOCKER;

async fn is_container_running(container_id: String) -> Result<bool, bollard::errors::Error> {
  event!(Level::DEBUG, "Inspecting if container {} is running", container_id.as_str());

  let result = DOCKER.inspect_container(&container_id, None).await;

  result.map(|res| res.state.map(|state| state.running.unwrap_or(false)).unwrap_or(false))
}

