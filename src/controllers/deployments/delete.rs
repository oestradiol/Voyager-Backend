use axum::{extract::Query, response::IntoResponse};
use std::collections::HashMap;

use crate::business::{
  self,
  services::{self, deployments},
};

pub async fn delete(Query(queries): Query<HashMap<String, String>>) -> impl IntoResponse {}
