use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn delete(name: String) -> Result<(), VoyagerError> {
  event!(
    Level::DEBUG,
    "Deleting deployment of name {name} from database."
  );

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::delete",
      DB_CONTEXT
        .deployments
        .delete_one(doc! {"name": &name}, None),
    )
    .await?;

  let result = result.map_or_else(
    |e| Err(VoyagerError::delete_mongo(Box::new(e), &name)),
    |r| {
      if r.deleted_count == 0 {
        Err(VoyagerError::delete(&name))
      } else {
        Ok(())
      }
    },
  );

  event!(
    Level::DEBUG,
    "Done deleting deployment."
  );

  result
}

impl VoyagerError {
  fn delete_mongo(e: Error, name: &str) -> Self {
    Self::new(
      format!("Failed to delete deployment named '{name}'"),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn delete(name: &str) -> Self {
    Self::new(
      format!("Deployment not found. Name: '{name}'"),
      StatusCode::NOT_FOUND,
      None,
    )
  }
}
