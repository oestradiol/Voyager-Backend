use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use futures::Future;
use std::collections::HashMap;

use crate::{
  business::{
    self,
    services::{self, deployments},
  },
  types::view::{get_deployment::GetDeployment, logs::Logs},
};

use crate::types::to_json_str;

pub async fn get(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("deploymentId").cloned();

  let inner = || async {
    let Some(id) = id_opt else {
      return (
        StatusCode::BAD_REQUEST,
        to_json_str(&GetDeployment {
          logs: Logs {
            message: "deploymentId is required".to_string(),
            errors: vec![],
          },
          deployment: None,
        }),
      );
    };

    match deployments::get(id).await {
      Ok(deployment) => (
        StatusCode::OK,
        to_json_str(&GetDeployment {
          logs: Logs {
            message: "Success!".to_string(),
            errors: vec![],
          },
          deployment: Some(deployment),
        }),
      ),
      Err(e) => (
        e.status_code,
        to_json_str(&GetDeployment {
          logs: Logs {
            message: "Failed to retrieve deployment".to_string(),
            errors: vec![e.message],
          },
          deployment: None,
        }),
      ),
    }
  };

  let res = inner().await;
  (
    res.0,
    res.1.unwrap_or_else(|_| {
      "{
  \"logs\": {
    \"message\": \"Internal Server Error\",
    \"errors\": []
  },
  \"deployment\": null,
}"
      .to_string()
    }),
  )
}
