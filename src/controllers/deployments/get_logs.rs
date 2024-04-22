use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse, Json,
};
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::view::{get_deployment_logs::GetDeploymentLogs, logs::Logs},
};


pub async fn get_logs(Path(queries): Path<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("id").cloned();

  let Some(id) = id_opt else {
    return (
      StatusCode::BAD_REQUEST,
      Json(GetDeploymentLogs {
        logs: Logs {
          message: "deploymentId is required".to_string(),
          errors: vec![],
        },
        deployment_logs: None,
      }),
    );
  };

  match deployments::get_logs(id).await {
    Ok(deployment_logs) => (
      StatusCode::OK,
      Json(GetDeploymentLogs {
        logs: Logs {
          message: "Success!".to_string(),
          errors: vec![],
        },
        deployment_logs: Some(deployment_logs),
      }),
    ),
    Err(e) => (
      e.status_code,
      Json(GetDeploymentLogs {
        logs: Logs {
          message: "Failed to retrieve deployment logs".to_string(),
          errors: vec![e.message],
        },
        deployment_logs: None,
      }),
    ),
  }
}
