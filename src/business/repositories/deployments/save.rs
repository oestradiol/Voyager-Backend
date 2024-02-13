use mongodb::bson::Bson;
use tracing::{event, Level};
use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::model::deployment::Deployment, Error,
  utils::runtime_helpers::RuntimeSpawnHandled
};

pub async fn save(deployment: Deployment) -> Option<Bson> {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future =
    async move {
      let result = DB_CONTEXT.deployments
        .insert_one(deployment, None).await;

      result.map_err(Error::from) // MongoDB Error
  };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::save", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
      None
    }, |f| Some(f.inserted_id))
  }).and_then(|id| id)
}
