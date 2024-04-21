use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use futures::Future;
use std::collections::HashMap;

use crate::{
  business::{
    self,
    services::{self, deployments},
  },
  types::{
    model::deployment::Mode,
    view::{create_deployment::CreateDeployment, get_deployment::GetDeployment, logs::Logs},
  },
};

use crate::types::to_json_str;

pub async fn create(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let repo_url = queries.get("repoUrl").cloned();
  let subdomain = queries.get("subdomain").cloned();
  let mode = queries.get("mode").cloned();

  let inner = || async {
    let Some(repo_url) = repo_url else {
      return (
        StatusCode::BAD_REQUEST,
        to_json_str(&GetDeployment {
          logs: Logs {
            message: "repoUrl is required".to_string(),
            errors: vec![],
          },
          deployment: None,
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
        to_json_str(&GetDeployment {
          logs: Logs {
            message: "mode is required".to_string(),
            errors: vec![],
          },
          deployment: None,
        }),
      );
    };
    let mode = match mode.as_str() {
      "production" => Mode::Production,
      "preview" => Mode::Preview,
      _ => {
        return (
          StatusCode::BAD_REQUEST,
          to_json_str(&GetDeployment {
            logs: Logs {
              message: "mode must be either 'production' or 'preview'".to_string(),
              errors: vec![],
            },
            deployment: None,
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
        to_json_str(&CreateDeployment {
          logs: Logs {
            message: "Success!".to_string(),
            errors: vec![],
          },
          id: Some(deployment_id),
        }),
      ),
      Err(e) => (
        e.status_code,
        to_json_str(&CreateDeployment {
          logs: Logs {
            message: "Failed to create deployment".to_string(),
            errors: vec![e.message],
          },
          id: None,
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
