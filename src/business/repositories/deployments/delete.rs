use mongodb::bson::doc;
use tracing::{event, Level};
use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  Error, utils::runtime_helpers::RuntimeSpawnHandled
};

pub async fn delete(id: String) -> bool {
  event!(Level::DEBUG, "Retrieving ALL deployments...");

  let future =
    async move {
      let result = DB_CONTEXT.deployments
        .delete_one(doc! {"_id": id}, None).await;

      result.map_err(Error::from) // MongoDB Error
  };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::delete", future).await;


  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
      false
    }, |d| d.deleted_count > 0)
  }).map_or(false, |f| f)
}
