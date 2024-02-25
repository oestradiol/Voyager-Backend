use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn find_by_id(id: String) -> Result<Deployment, VoyagerError> {
  event!(Level::DEBUG, "Finding deployment with id {}", &id);

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::find_by_id",
      DB_CONTEXT.deployments.find_one(doc! { "_id": &id }, None),
    )
    .await?;

  result.map_or_else(
    |e| Err(VoyagerError::find_mongo_id(Box::new(e), &id)),
    |r| r.ok_or_else(|| VoyagerError::find_null_id(&id)),
  )
}

impl VoyagerError {
  fn find_mongo_id(e: Error, id: &str) -> Self {
    Self::new(
      format!("Failed to find deployment by id '{id}'"),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn find_null_id(id: &str) -> Self {
    Self::new(
      format!("Deployment not found. Id: '{id}'"),
      StatusCode::NOT_FOUND,
      None,
    )
  }
}
