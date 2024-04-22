use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::view::{get_deployment::GetDeployment, logs::Logs},
};

pub async fn get(Path(queries): Path<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("id").cloned();

  let Some(id) = id_opt else {
    return (
      StatusCode::BAD_REQUEST,
      Json(GetDeployment {
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
      Json(GetDeployment {
        logs: Logs {
          message: "Success!".to_string(),
          errors: vec![],
        },
        deployment: Some(deployment),
      }),
    ),
    Err(e) => (
      e.status_code,
      Json(GetDeployment {
        logs: Logs {
          message: "Failed to retrieve deployment".to_string(),
          errors: vec![e.message],
        },
        deployment: None,
      }),
    ),
  }
}
