use tracing::{event, Level};

use crate::{
  business::{repositories, services::SERVICES_RUNTIME},
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
  utils::runtime_helpers::RuntimeSpawnHandled,
};

pub async fn list(
  repo_url: Option<String>,
  branch: Option<String>,
) -> Result<Vec<Deployment>, VoyagerError> {
  let mut log = "Retrieving deployments".to_string();
  if let Some(repo) = repo_url.clone() {
    log = format!("{log}. Repo: {repo}");
  }
  if let Some(branch) = branch.clone() {
    log = format!("{log}. Branch: {branch}");
  }
  event!(Level::INFO, log);

  SERVICES_RUNTIME
    .spawn_handled(
      "services::deployments::list",
      repositories::deployments::retrieve_all(repo_url, branch),
    )
    .await?
}
