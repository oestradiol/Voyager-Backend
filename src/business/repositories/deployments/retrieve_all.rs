use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::{runtime_helpers::RuntimeSpawnHandled, Error},
};
use axum::http::StatusCode;
use mongodb::bson::doc;
use tracing::{event, Level};

pub async fn retrieve_all(
  repo_url: Option<String>,
  branch: Option<String>,
) -> Result<Vec<Deployment>, VoyagerError> {
  let repo_and_branch = branch.as_ref().map_or(String::new(), |b| format!("@{b}"));
  let repo_and_branch = repo_url.as_ref()
    .map_or(String::new(), |r| format!("{r}{repo_and_branch}"));

  event!(
    Level::DEBUG,
    "Retrieving deployments from database: {repo_and_branch}"
  );

  let future = async move {
    let document = repo_url
      .map_or_else(|| doc! { }, |repo_url|
        branch.map_or_else(|| doc! {"repo_url": &repo_url}, |branch|
          doc! {"repo_url": &repo_url, "branch": &branch}
        )
      );

    let result = DB_CONTEXT
      .deployments
      .find(document, None)
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

  event!(Level::DEBUG, "Done retrieving deployments");

  Ok(result)
}

impl VoyagerError {
  fn retrieve_all(e: Error) -> Self {
    Self::new(
      "Failed to retrieve deployments".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      false,
      Some(e),
    )
  }
}
