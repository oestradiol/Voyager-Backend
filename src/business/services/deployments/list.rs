use crate::{
  business::repositories,
  types::{model::deployment::Deployment, other::voyager_error::VoyagerError},
};

pub async fn list(
  repo_url: Option<String>,
  branch: Option<String>,
) -> Result<Vec<Deployment>, VoyagerError> {
  repositories::deployments::retrieve_all(repo_url, branch).await
}
