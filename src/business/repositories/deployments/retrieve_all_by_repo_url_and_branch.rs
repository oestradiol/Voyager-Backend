use mongodb::{bson::doc, Cursor};
use tracing::{event, Level};
use crate::{
    business::repositories::{DB_CONTEXT, REPOSITORIES_RUNTIME},
    types::model::deployment::Deployment, Error,
    utils::runtime_helpers::RuntimeSpawnHandled
};

pub async fn retrieve_all_by_repo_url_and_branch(repo_url: String, branch: Option<String>) -> Option<Cursor<Deployment>> {
  let repo_and_branch = branch.clone().map_or("".to_string(), |b| format!("@{b}"));
  let repo_and_branch = format!("{}{}", repo_url, repo_and_branch);
  event!(Level::DEBUG, "Retrieving deployments from {repo_and_branch}");

  let future =
    async move {
      let result = DB_CONTEXT.deployments
        .find(doc! {"repo_url": repo_url, "branch": branch}, None).await;

      result.map_err(Error::from) // MongoDB Error
    };

  let result = REPOSITORIES_RUNTIME.spawn_handled("repositories::deployments::retrieve_all_by_repo_url_and_branch", future).await;

  result.map(|r| {
    r.map_or_else(|e| {
      event!(Level::ERROR, "Failed to retrieve deployments for {repo_and_branch}: {}", e);
      None
    }, |c| Some(c))
  }).and_then(|c| c)
}
