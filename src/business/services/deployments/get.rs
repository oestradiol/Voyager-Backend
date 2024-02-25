use crate::business::repositories;
use crate::types::model::deployment::Deployment;
use crate::types::other::voyager_error::VoyagerError;

pub async fn get(id: String) -> Result<Deployment, VoyagerError> {
  repositories::deployments::find_by_id(id).await
}
