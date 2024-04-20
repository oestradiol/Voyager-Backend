use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use futures::Future;
use std::collections::HashMap;

use crate::{
  business::{
    self,
    services::{self, deployments},
  },
  types::view::{get_deployments::GetDeployments, logs::Logs},
};

use crate::types::to_json_str;
pub async fn list(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let repo_url = queries.get("repoUrl").cloned();
  let branch = queries.get("branch").cloned();

  let inner = || async {
    match deployments::list(repo_url, branch).await {
      Ok(deployments) => (
        StatusCode::OK,
        to_json_str(&GetDeployments {
          logs: Logs {
            message: "Success!".to_string(),
            errors: vec![],
          },
          deployments,
        }),
      ),
      Err(e) => (
        e.status_code,
        to_json_str(&GetDeployments {
          logs: Logs {
            message: "Failed to retrieve deployments".to_string(),
            errors: vec![e.message],
          },
          deployments: vec![],
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
