use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::view::{get_deployments::GetDeployments, logs::Logs},
};

pub async fn list(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let repo_url = queries.get("repoUrl").cloned();
  let branch = queries.get("branch").cloned();

  match deployments::list(repo_url, branch).await {
    Ok(deployments) => (
      StatusCode::OK,
      Json(GetDeployments {
        logs: Logs {
          message: "Success!".to_string(),
          errors: vec![],
        },
        deployments,
      }),
    ),
    Err(e) => (
      e.status_code,
      Json(GetDeployments {
        logs: Logs {
          message: "Failed to retrieve deployments".to_string(),
          errors: vec![e.message],
        },
        deployments: vec![],
      }),
    ),
  }
}
