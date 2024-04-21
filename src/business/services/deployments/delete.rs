use tracing::{event, Level};

use crate::{
  business::{repositories, services::SERVICES_RUNTIME},
  modules::{
    cloudflare::delete_dns_record,
    docker::{self, delete_container, delete_image, is_container_running},
  },
  types::other::voyager_error::VoyagerError,
  utils::runtime_helpers::RuntimeSpawnHandled,
};

pub async fn delete(deployment_id: String) -> Result<(), VoyagerError> {
  event!(Level::INFO, "Deleting deployment: {}", &deployment_id);

  let future = async move {
    let deployment = repositories::deployments::find_by_id(deployment_id.clone()).await?;
    let name = deployment.container_name;

    if is_container_running(name.clone()).await? {
      docker::stop_container(name.clone()).await?;
    }

    delete_dns_record(&deployment.dns_record_id).await?;
    
    delete_container(name.clone()).await?;
    delete_image(deployment.image_name).await?;

    repositories::deployments::delete(deployment_id).await?;

    // TODO: notify user via email

    Ok(())
  };

  let result = SERVICES_RUNTIME
    .spawn_handled("services::deployments::delete", future)
    .await?;

  event!(Level::DEBUG, "Done deleting deployment.");

  result
}