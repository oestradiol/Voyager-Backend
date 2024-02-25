use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use futures::{executor::block_on, future::OptionFuture, TryFutureExt};
use mongodb::{bson::doc, Cursor};
use tracing::{event, Level};

pub async fn retrieve_all(
  repo_url: Option<String>,
  branch: Option<String>,
) -> Result<Vec<Deployment>, VoyagerError> {
  let repo_and_branch = branch.clone().map_or(String::new(), |b| format!("@{b}"));
  let repo_and_branch = repo_url
    .clone()
    .map_or(String::new(), |r| format!("{r}{repo_and_branch}"));

  event!(
    Level::DEBUG,
    "Retrieving deployments from {repo_and_branch}"
  );

  let future = async move {
    let result = DB_CONTEXT
      .deployments
      // TODO: test for null value
      .find(doc! {"repo_url": repo_url, "branch": branch}, None)
      .await
      .map_or_else(
        |e| Err(VoyagerError::retrieve_all(Box::new(e))),
        |mut cursor| {
          Ok(async move {
            let mut list = Vec::new();
            while cursor.advance().await.unwrap_or(false) {
              if let Ok(crr) = cursor.deserialize_current() {
                list.push(crr);
              }
            }
            list
          })
        },
      )?;

    Ok(result.await)
  };

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::retrieve_all_by_repo_url_and_branch",
      future,
    )
    .await??;

  event!(Level::DEBUG, "Done retrieving deployments.");

  Ok(result)
}

impl VoyagerError {
  fn retrieve_all(e: Error) -> Self {
    Self::new(
      "Failed to retrieve deployments".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }
}
