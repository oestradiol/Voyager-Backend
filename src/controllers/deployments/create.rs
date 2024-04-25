use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use regex::Regex;
use std::collections::HashMap;

use crate::{
  business::services::deployments,
  types::{
    model::deployment::Mode, view::{create_deployment::CreateDeployment, logs::Logs}
  },
};

pub async fn create(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let mode = queries.get("mode").cloned();
  let repo_url = queries.get("repoUrl").cloned();
  let subdomain = queries.get("subdomain").cloned();

  // Validations
  let mode = match mode.as_deref() {
    Some("production") => Mode::Production,
    Some("preview") => Mode::Preview,
    Some(_) => {
      return (
        StatusCode::BAD_REQUEST,
        Json(CreateDeployment {
          logs: Logs {
            message: "Mode must be either 'production' or 'preview'".to_string(),
            errors: vec![],
          },
          id: None,
        }),
      );
    },
    None => return (
      StatusCode::BAD_REQUEST,
      Json(CreateDeployment {
        logs: Logs {
          message: "Mode is required".to_string(),
          errors: vec![],
        },
        id: None,
      }),
    ),
  };
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
  let host = match resolve_host(subdomain, &mode) {
    None => return (
      StatusCode::BAD_REQUEST,
      Json(CreateDeployment {
        logs: Logs {
          message: "Subdomains can only have alphanumerics, underscore and dashes, and can only start with alphanumerics.".to_string(),
          errors: vec![],
        },
        id: None,
      }),
    ),
    Some(host) => host
  };

  let split = repo_url.split('@').collect::<Vec<_>>();
  let repo_url = split[0].to_string();
  let branch = split.get(1).map(std::string::ToString::to_string);

  match async {
    deployments::check(host.clone(), mode, repo_url.clone(), branch.clone()).await?;
    deployments::new(host, mode, repo_url, branch).await
  }.await {
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

fn resolve_host(subdomain: Option<String>, mode: &Mode) -> Option<String> {
  let subdomain = subdomain.unwrap_or_default();
  
  // Validates the subdomain
  #[allow(clippy::unwrap_used)] // We know that the unwrap will always succeed because it is a valid Regex
  let re = Regex::new(r"^$|[a-zA-Z0-9][a-zA-Z0-9_-]*").unwrap();
  let is_valid = if let Some(caps) = re.captures(&subdomain) {
    caps[0].len().eq(&subdomain.len())
  } else { false };
  if !is_valid {
    return None;
  }

  // Processes the hostname
  let mut host = if !subdomain.is_empty() {
    if matches!(mode, Mode::Preview) {
      format!("{subdomain}-")
    } else {
      format!("{subdomain}.")
    }
  } else {
    String::new()
  };
  if matches!(mode, Mode::Preview) {
    host = format!("{host}preview.");
  }
  host = format!("{host}lunarlabs.cc");

  Some(host)
}
