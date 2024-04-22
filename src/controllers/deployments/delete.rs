use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::view::{delete_deployment::DeleteDeployment, logs::Logs},
};

pub async fn delete(Path(queries): Path<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("id").cloned();

  let Some(id) = id_opt else {
    return (
      StatusCode::BAD_REQUEST,
      Json(DeleteDeployment {
        logs: Logs {
          message: "deploymentId is required".to_string(),
          errors: vec![],
        },
      }),
    );
  };

  match deployments::delete(id).await {
    Ok(()) => (
      StatusCode::OK,
      Json(DeleteDeployment {
        logs: Logs {
          message: "Success!".to_string(),
          errors: vec![],
        },
      }),
    ),
    Err(e) => (
      e.status_code,
      Json(DeleteDeployment {
        logs: Logs {
          message: "Failed to delete deployment".to_string(),
          errors: vec![e.message],
        },
      }),
    ),
  }
}
