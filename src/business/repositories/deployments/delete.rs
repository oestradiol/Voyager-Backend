use std::str::FromStr;

use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::other::voyager_error::VoyagerError,
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::{doc, oid::ObjectId};
use tracing::{event, Level};

pub async fn delete(id: String) -> Result<(), VoyagerError> {
  event!(
    Level::DEBUG,
    "Deleting deployment of id {id} from database."
  );

  let oid = ObjectId::from_str(id.as_str())
    .map_err(|e| VoyagerError::invalid_delete_id(Box::new(e), &id))?;

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::delete",
      DB_CONTEXT
        .deployments
        .delete_one(doc! { "_id": oid }, None),
    )
    .await?;

  let result = result.map_or_else(
    |e| Err(VoyagerError::delete_mongo(Box::new(e), &id)),
    |r| {
      if r.deleted_count == 0 {
        Err(VoyagerError::delete(&id))
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
  fn invalid_delete_id(e: Error, id: &str) -> Self {
    Self::new(
      format!("Invalid Bson id '{id}'"),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }

  fn delete_mongo(e: Error, id: &str) -> Self {
    Self::new(
      format!("Failure while deleting deployment with id '{id}'"),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }

  fn delete(id: &str) -> Self {
    Self::new(
      format!("Deployment not found. Id: '{id}'"),
      StatusCode::NOT_FOUND,
      false,
      None,
    )
  }
}
