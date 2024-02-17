use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn delete(name: String) -> Option<()> {
  event!(
    Level::DEBUG,
    "Deleting deployment of name {name} from database."
  );

  let future = async move {
    let result = DB_CONTEXT
      .deployments
      .delete_one(doc! {"name": name}, None)
      .await;

    result.map_err(Error::from) // MongoDB Error
  };

  let result = REPOSITORIES_RUNTIME
    .spawn_handled("repositories::deployments::delete", future)
    .await;

  result
    .map(|r| {
      r.map_or_else(
        |e| {
          event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
          false
        },
        |d| d.deleted_count > 0,
      )
    })
    .map(|r| if r { Some(()) } else { None })
    .and_then(|r| r)
}
