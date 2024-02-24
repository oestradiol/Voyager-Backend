use crate::types::model::deployment::Deployment;
use crate::business::repositories;


pub async fn get(id: String) -> Option<Deployment> {
  repositories::deployments::find_by_id(id).await
}
