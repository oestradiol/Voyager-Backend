use mongodb::{bson::doc, Cursor};
use tracing::{event, Level};
use crate::{
  business::repositories::{APP_DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::model::deployment::Deployment, Error,
  utils::runtime_helpers::RuntimeSpawnHandled
};

pub async fn retrieve_all() -> Option<Cursor<Deployment>> {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future = 
    async move {
      let result = APP_DB_CONTEXT.deployments
        .find(doc! {}, None).await;

      result.map_err(Error::from) // MongoDB Error
    };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::retrieve_all", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
      None
    }, |c| Some(c))
  }).and_then(|c| c)
}