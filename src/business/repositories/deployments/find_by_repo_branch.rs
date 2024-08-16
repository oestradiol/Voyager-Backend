use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn find_by_repo_branch(repo_url: &str, branch: &str) -> Result<Option<Deployment>, VoyagerError> {
  event!(
    Level::DEBUG,
    "Finding deployment with repo url {} and branch {} in database",
    repo_url,
    branch
  );

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::find_by_repo_branch",
      DB_CONTEXT.deployments.find_one(doc! { "repo_url": repo_url, "branch": branch }, None),
    )
    .await?;

  let result = result.map_or_else(
    |e| Err(VoyagerError::find_mongo_repo_branch(Box::new(e), repo_url, branch)),
    |r| Ok(r),
  )?;

  event!(Level::DEBUG, "Done finding deployment");

  Ok(result)
}

impl VoyagerError {
  fn find_mongo_repo_branch(e: Error, repo_url: &str, branch: &str) -> Self {
    Self::new(
      format!("Failure while finding deployment by repo url '{repo_url}' and branch {branch}"),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }
}
