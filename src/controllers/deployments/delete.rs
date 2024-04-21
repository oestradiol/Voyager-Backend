use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::view::{delete_deployment::DeleteDeployment, logs::Logs},
};

use crate::types::to_json_str;

pub async fn delete(Path(queries): Path<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("id").cloned();

  let inner = || async {
    let Some(id) = id_opt else {
      return (
        StatusCode::BAD_REQUEST,
        to_json_str(&DeleteDeployment {
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
        to_json_str(&DeleteDeployment {
          logs: Logs {
            message: "Success!".to_string(),
            errors: vec![],
          },
        }),
      ),
      Err(e) => (
        e.status_code,
        to_json_str(&DeleteDeployment {
          logs: Logs {
            message: "Failed to delete deployment".to_string(),
            errors: vec![e.message],
          },
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
