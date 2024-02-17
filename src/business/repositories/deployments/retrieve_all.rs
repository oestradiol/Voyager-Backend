use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::model::deployment::Deployment,
  utils::runtime_helpers::RuntimeSpawnHandled,
  utils::Error,
};
use mongodb::{bson::doc, Cursor};
use tracing::{event, Level};

pub async fn retrieve_all() -> Option<Cursor<Deployment>> {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future = async move {
    let result = DB_CONTEXT.deployments.find(doc! {}, None).await;

    result.map_err(Error::from) // MongoDB Error
  };

  let result = REPOSITORIES_RUNTIME
    .spawn_handled("repositories::deployments::retrieve_all", future)
    .await;

  result
    .map(|r| {
      r.map_or_else(
        |e| {
          event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
          None
        },
        Some,
      )
    })
    .and_then(|c| c)
}
