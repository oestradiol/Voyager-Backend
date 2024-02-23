use crate::{
  business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
  types::model::deployment::Deployment,
  utils::runtime_helpers::RuntimeSpawnHandled,
  utils::Error,
};
use futures::{executor::block_on, future::OptionFuture};
use mongodb::{bson::doc, Cursor};
use tracing::{event, Level};

pub async fn retrieve_all(
  repo_url: Option<String>,
  branch: Option<String>,
) -> Option<Vec<Deployment>> {
  let repo_and_branch = branch.clone().map_or(String::new(), |b| format!("@{b}"));
  let repo_and_branch = repo_url.clone().map_or(String::new(), |r| format!("{r}{repo_and_branch}"));
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
        |e| {
          event!(Level::ERROR, "Failed to retrieve deployments: {}", e);
          None
        },
        |mut cursor| Some(async move {
          let mut list = Vec::new();
          while cursor.advance().await.unwrap_or(false) {
            if let Ok(crr) = cursor.deserialize_current() {
              list.push(crr);
            }
          }
          list
        }),
      );

    OptionFuture::from(result).await
  };

  let result = REPOSITORIES_RUNTIME
    .spawn_handled(
      "repositories::deployments::retrieve_all_by_repo_url_and_branch",
      future,
    )
    .await;

  result.and_then(|c| c)
}
