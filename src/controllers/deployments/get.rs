use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use futures::Future;
use std::collections::HashMap;

use crate::{
  business::{
    self,
    services::{self, deployments},
  },
  types::view::get_deployment::GetDeployment,
};

use crate::types::to_json_str;

pub async fn get(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {
  let id_opt = queries.get("deploymentId").cloned();

  let inner = || async {
    let Some(id) = id_opt else {
      return Some((
        StatusCode::BAD_REQUEST,
        to_json_str(&GetDeployment { deployment: None })?,
      ));
    };

    match deployments::get(id).await {
      Some(deployment) => Some((
        StatusCode::OK,
        to_json_str(&GetDeployment {
          deployment: Some(deployment),
        })?,
      )),
      None => Some((
        StatusCode::NOT_FOUND,
        to_json_str(&GetDeployment { deployment: None })?,
      )),
    }
  };

  inner().await.unwrap_or((
    StatusCode::INTERNAL_SERVER_ERROR,
    "Internal Server Error".to_string(),
  ))
}
