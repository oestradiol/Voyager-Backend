use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn find_by_name(name: String) -> Result<Deployment, VoyagerError> {
  event!(Level::DEBUG, "Finding deployment with name {}", &name);

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::find_by_name",
      DB_CONTEXT
        .deployments
        .find_one(doc! { "name": &name }, None),
    )
    .await?;

  result.map_or_else(
    |e| Err(VoyagerError::find_mongo_name(Box::new(e), &name)),
    |r| r.ok_or_else(|| VoyagerError::find_null_name(&name)),
  )
}

impl VoyagerError {
  fn find_mongo_name(e: Error, name: &str) -> Self {
    let message = format!("Failed to find deployment named '{name}'! Error:{e}");

    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }

  fn find_null_name(name: &str) -> Self {
    let message = format!("Failed to find deployment named '{name}'!");

    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::NOT_FOUND,
      source: None,
    }
  }
}
