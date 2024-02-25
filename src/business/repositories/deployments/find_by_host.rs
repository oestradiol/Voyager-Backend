use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn find_by_host(host: String) -> Result<Deployment, VoyagerError> {
  event!(Level::DEBUG, "Finding deployment by host {}", &host);

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::find_by_host",
      DB_CONTEXT
        .deployments
        .find_one(doc! { "host": &host }, None),
    )
    .await?;

  result.map_or_else(
    |e| Err(VoyagerError::find_mongo_host(Box::new(e), &host)),
    |r| r.ok_or_else(|| VoyagerError::find_null_host(&host)),
  )
}

impl VoyagerError {
  fn find_mongo_host(e: Error, host: &str) -> Self {
    let message = format!("Failed to find deployment by host '{host}'! Error:{e}");

    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      source: Some(e),
    }
  }

  fn find_null_host(host: &str) -> Self {
    let message = format!("Failed to find deployment by host '{host}'!");

    event!(Level::ERROR, message);
    Self {
      message,
      status_code: StatusCode::NOT_FOUND,
      source: None,
    }
  }
}
