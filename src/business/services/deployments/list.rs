use crate::{business::repositories, types::model::deployment::Deployment};


pub async fn list(repo_url: Option<String>, branch: Option<String>) -> Option<Vec<Deployment>> {
  repositories::deployments::retrieve_all(repo_url, branch).await
}
