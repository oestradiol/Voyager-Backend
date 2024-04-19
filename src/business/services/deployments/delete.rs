use std::{fs, path::PathBuf};

use axum::http::StatusCode;
use tracing::{event, Level};

use crate::{
  business::{repositories, services::SERVICES_RUNTIME},
  configs::environment::DEPLOYMENTS_DIR,
  modules::{
    cloudflare::remove_dns_record,
    docker::{delete_container, delete_image, is_container_running},
  },
  types::{
    model::deployment::Deployment, other::voyager_error::VoyagerError,
    view::delete_deployment::DeleteDeployment,
  },
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

async fn delete(deployment: Deployment) -> Result<(), VoyagerError> {
  event!(
    Level::INFO,
    "Deleting deployment: {}",
    &deployment.container_name
  );

  let future = async move {
    let name = deployment.container_name;

    if is_container_running(name.clone()).await? {
      return Err(VoyagerError::delete_running());
    }

    delete_image(deployment.image_name).await?;
    delete_container(name.clone()).await?;
    remove_dns_record(&deployment.dns_record_id).await?;

    repositories::deployments::delete(name).await?;

    tokio::fs::remove_dir_all(PathBuf::from(&*DEPLOYMENTS_DIR).join(&deployment.directory))
      .await
      .map_err(|e| VoyagerError::delete_dir(Box::new(e)))?;

    // TODO: notify user via email

    Ok(())
  };

  SERVICES_RUNTIME
    .spawn_handled("services::deployments::delete", future)
    .await?
}

impl VoyagerError {
  fn delete_dir(e: Error) -> Self {
    Self::new(
      "Failed to delete deployment directory".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn delete_running() -> Self {
    Self::new(
      "Tried to delete container that is running".to_string(),
      StatusCode::BAD_REQUEST,
      None,
    )
  }
}
