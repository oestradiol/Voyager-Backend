use tracing::{event, Level};

use crate::business::repositories;
use crate::business::services::SERVICES_RUNTIME;
use crate::modules::docker;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::runtime_helpers::RuntimeSpawnHandled;

pub async fn get_logs(id: String) -> Result<Vec<String>, VoyagerError> {
  event!(Level::INFO, "Retrieving deployment logs. Id: {id}");

  let future = async move {
    let deployment = repositories::deployments::find_by_id(id).await?;

    docker::get_logs(&deployment.container_name).await
  };

  let result = SERVICES_RUNTIME
    .spawn_handled("services::deployments::get_logs", future)
    .await?;

  event!(Level::DEBUG, "Done retrieving deployment logs.");

  result
}
