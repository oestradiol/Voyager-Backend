use tracing::{event, Level};

use crate::business::repositories;
use crate::business::services::SERVICES_RUNTIME;
use crate::types::model::deployment::Deployment;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::runtime_helpers::RuntimeSpawnHandled;

pub async fn get(id: String) -> Result<Deployment, VoyagerError> {
  event!(Level::INFO, "Retrieving deployment. Id: {id}");

  let result = SERVICES_RUNTIME
    .spawn_handled(
      "services::deployments::get",
      async move { repositories::deployments::find_by_id(&id).await },
    )
    .await?;

  event!(Level::DEBUG, "Done retrieving deployment.");

  result
}
