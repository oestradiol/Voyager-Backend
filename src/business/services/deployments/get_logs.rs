use crate::business::repositories;
use crate::modules::docker;
use crate::types::other::voyager_error::VoyagerError;

pub async fn get_logs(id: String) -> Result<Vec<String>, VoyagerError> {
  let deployment = repositories::deployments::find_by_id(id).await?;

  docker::get_logs(&deployment.container_name).await
}
