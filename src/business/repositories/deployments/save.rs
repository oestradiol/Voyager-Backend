use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::Bson;
use tracing::{event, Level};

pub async fn save(deployment: Deployment) -> Result<Bson, VoyagerError> {
  event!(Level::DEBUG, "Saving deployment...");

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::save",
      DB_CONTEXT.deployments.insert_one(deployment, None),
    )
    .await?;

  let result = result.map_or_else(
    |e| Err(VoyagerError::save(Box::new(e))),
    |r| Ok(r.inserted_id),
  )?;

  event!(Level::DEBUG, "Done saving deployment.");

  Ok(result)
}

impl VoyagerError {
  fn save(e: Error) -> Self {
    Self::new(
      "Failed to save deployment".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
