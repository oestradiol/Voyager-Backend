use serde::Deserialize;
use crate::modules::cloudflare::types::cloudflare_responses::{CloudflareError, CloudflareMessage};

#[derive(Debug, Deserialize)]
pub struct Success {
  pub errors: Vec<CloudflareError>,
  pub messages: Vec<CloudflareMessage>,
  pub result: Data,
  pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Data {
  pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Failure {
  pub errors: Vec<CloudflareError>,
  pub messages: Vec<CloudflareMessage>,
  pub success: bool,
}
