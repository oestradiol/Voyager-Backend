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

  result.map_or_else(
    |e| Err(VoyagerError::delete_mongo(Box::new(e), &name)),
    |r| {
      if r.deleted_count == 0 {
        Err(VoyagerError::delete(&name))
      } else {
        Ok(())
      }
    },
  )
}

impl VoyagerError {
  pub fn delete_mongo(e: Error, name: &str) -> Self {
    let message = format!("Failed to delete deployment named '{name}'! Error:{e}");

    event!(Level::ERROR, message);
    VoyagerError {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }

  pub fn delete(name: &str) -> Self {
    let message = format!("Failed to find deployment named '{name}'!");

    event!(Level::ERROR, message);
    VoyagerError {
      message,
      status_code: StatusCode::NOT_FOUND,
      source: None,
    }
  }
}
