use tracing::{event, Level};

use crate::{
  business::{repositories, services::SERVICES_RUNTIME},
  modules::{
    cloudflare::remove_dns_record,
    docker::{delete_container, delete_image},
  },
  types::{model::deployment::Deployment, view::delete_deployment::DeleteDeployment},
  utils::{runtime_helpers::RuntimeSpawnHandled, to_Ok, Error, ResultEx},
};

async fn delete(deployment: Deployment) -> Option<()> {
  event!(Level::INFO, "Deleting deployment: {}", &deployment.name);

  let future = async move {
    // if state != DeploymentState::STOPPED {
    //     event!(Level::ERROR "Deployment is running");
    //     return Res(Err(Error::new("Tried to delete deployment that is not in stopped state: $deployment")));
    // }

    let name = deployment.name;

    delete_image(name.clone()).await?;
    delete_container(name.clone()).await?;

    // File::new(directory).await?.also(|it| {
    //     log("Checking if directory for deployment with id $id exists before deleting", LogType::DEBUG);
    //     if it.exists() {
    //         log("It exists, deleting..", LogType::DEBUG);
    //         it.delete_recursively().await?;
    //     }
    // });

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
