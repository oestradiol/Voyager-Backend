use tracing::{event, Level};

use crate::{
  business::{repositories, services::SERVICES_RUNTIME},
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
      event!(Level::ERROR, "Tried to delete container that is running");
      return None;
    }

    delete_image(name.clone()).await?;
    delete_container(name.clone()).await?;

    repositories::deployments::delete(name).await?;

    remove_dns_record(&deployment.dns_record_id).await?;

    // TODO: notify user via email

    Some(())
  };

  SERVICES_RUNTIME
    .spawn_handled("services::deployments::delete", future)
    .await
    .and_then(|f| f)
}
