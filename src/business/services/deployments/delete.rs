use std::{fs, path::PathBuf};

use axum::http::StatusCode;
use tracing::{event, Level};

use crate::{
  business::{repositories, services::SERVICES_RUNTIME},
  configs::environment::DEPLOYMENTS_DIR,
  modules::{
    cloudflare::delete_dns_record,
    docker::{delete_container, delete_image, is_container_running},
  },
  types::{
    model::deployment::Deployment, other::voyager_error::VoyagerError,
    view::delete_deployment::DeleteDeployment,
  },
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};

pub async fn delete(deployment_id: String) -> Result<(), VoyagerError> {
  event!(Level::INFO, "Deleting deployment: {}", &deployment_id);

  let future = async move {
    let deployment = repositories::deployments::find_by_id(deployment_id).await?;
    let name = deployment.container_name;

    if is_container_running(name.clone()).await? {
      return Err(VoyagerError::delete_running());
    }

    delete_dns_record(&deployment.dns_record_id).await?;
    
    delete_container(name.clone()).await?;
    delete_image(deployment.image_name).await?;

    repositories::deployments::delete(name).await?;

    // TODO: notify user via email

    Ok(())
  };

  let result = SERVICES_RUNTIME
    .spawn_handled("services::deployments::delete", future)
    .await?;

  event!(Level::DEBUG, "Done deleting deployment.");

  result
}

impl VoyagerError {
  fn delete_running() -> Self {
    Self::new(
      "Tried to delete container that is running".to_string(),
      StatusCode::BAD_REQUEST,
      None,
    )
  }
}
