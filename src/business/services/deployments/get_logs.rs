use crate::business::repositories;
use crate::modules::docker;


pub async fn get_logs(id: String) -> Option<Vec<String>> {
  if let Some(deployment) = repositories::deployments::find_by_id(id).await {
    return docker::get_logs(&deployment.container_name).await
  }

  None
}
