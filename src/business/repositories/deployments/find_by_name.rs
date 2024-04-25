use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn find_by_name(name: String) -> Result<Option<Deployment>, VoyagerError> {
  event!(
    Level::DEBUG,
    "Finding deployment with hostname {} in database",
    &name
  );

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::find_by_name",
      DB_CONTEXT.deployments.find_one(doc! { "container_name": &name }, None),
    )
    .await?;

  let result = result.map_or_else(
    |e| Err(VoyagerError::find_mongo_name(Box::new(e), &name)),
    |r| Ok(r),
  )?;

  event!(Level::DEBUG, "Done finding deployment");

  Ok(result)
}

impl VoyagerError {
  fn find_mongo_name(e: Error, name: &str) -> Self {
    Self::new(
      format!("Failure while finding deployment by hostname '{name}'"),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }
}
