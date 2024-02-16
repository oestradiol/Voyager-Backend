use super::DOCKER;

async fn restart_container(container_id: String) -> Result<(), bollard::errors::Error> {
  DOCKER.restart_container(&container_id, None).await
}

