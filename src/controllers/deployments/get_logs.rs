use axum::{
  extract::{Path, Query},
  http::StatusCode,
  response::IntoResponse,
};
use futures::Future;
use std::collections::HashMap;

use crate::{
  business::{
    self,
    services::{self, deployments},
  },
  types::view::{get_deployment_logs::GetDeploymentLogs, logs::Logs},
};

use crate::types::to_json_str;

pub async fn get_logs(Path(queries): Path<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("id").cloned();

  let inner = || async {
    let Some(id) = id_opt else {
      return (
        StatusCode::BAD_REQUEST,
        to_json_str(&GetDeploymentLogs {
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
        to_json_str(&GetDeploymentLogs {
          logs: Logs {
            message: "Success!".to_string(),
            errors: vec![],
          },
          deployment_logs: Some(deployment_logs),
        }),
      ),
      Err(e) => (
        e.status_code,
        to_json_str(&GetDeploymentLogs {
          logs: Logs {
            message: "Failed to retrieve deployment logs".to_string(),
            errors: vec![e.message],
          },
          deployment_logs: None,
        }),
      ),
    }
  };

  let res = inner().await;
  (
    res.0,
    res.1.unwrap_or_else(|_| {
      "{\"logs\":{\"message\":\"Internal Server Error\",\"errors\":[]},\"deployment\":null}"
        .to_string()
    }),
  )
}
