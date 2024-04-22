use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::{
    model::deployment::Mode,
    view::{create_deployment::CreateDeployment, logs::Logs},
  },
};

pub async fn create(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let repo_url = queries.get("repoUrl").cloned();
  let subdomain = queries.get("subdomain").cloned();
  let mode = queries.get("mode").cloned();

  let Some(repo_url) = repo_url else {
    return (
      StatusCode::BAD_REQUEST,
      Json(CreateDeployment {
        logs: Logs {
          message: "repoUrl is required".to_string(),
          errors: vec![],
        },
        id: None,
      }),
    );
  };
  let split = repo_url.split('@').collect::<Vec<_>>();
  #[allow(clippy::unwrap_used)] // We know that the split will always have at least one element
  let repo_url = (*split.first().unwrap()).to_string();
  let branch = split.get(1).map(std::string::ToString::to_string);

  let Some(mode) = mode else {
    return (
      StatusCode::BAD_REQUEST,
      Json(CreateDeployment {
        logs: Logs {
          message: "mode is required".to_string(),
          errors: vec![],
        },
        id: None,
      }),
    );
  };
  let mode = match mode.as_str() {
    "production" => Mode::Production,
    "preview" => Mode::Preview,
    _ => {
      return (
        StatusCode::BAD_REQUEST,
        Json(CreateDeployment {
          logs: Logs {
            message: "mode must be either 'production' or 'preview'".to_string(),
            errors: vec![],
          },
          id: None,
        }),
      );
    }
  };

  let mut host: String = String::new();
  if let Some(subdomain) = subdomain {
    if matches!(mode, Mode::Preview) {
      host = format!("{subdomain}-");
    } else {
      host = format!("{subdomain}.");
    }
  }
  if matches!(mode, Mode::Preview) {
    host = format!("{host}preview.");
  }
  host = format!("{host}lunarlabs.cc");

  match deployments::new(host, mode, repo_url, branch).await {
    Ok(deployment_id) => (
      StatusCode::OK,
      Json(CreateDeployment {
        logs: Logs {
          message: "Success!".to_string(),
          errors: vec![],
        },
        id: Some(deployment_id),
      }),
    ),
    Err(e) => (
      e.status_code,
      Json(CreateDeployment {
        logs: Logs {
          message: "Failed to create deployment".to_string(),
          errors: vec![e.message],
        },
        id: None,
      }),
    ),
  }
}
