use serde::Deserialize;
use crate::modules::cloudflare::types::cloudflare_responses::CloudflareError;

#[derive(Debug, Deserialize)]
pub struct OkResponse {
  // pub errors: Vec<CloudflareError>,
  // pub messages: Vec<CloudflareMessage>,
  pub result: Option<Data>,
  // pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Data {
  pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct FailureResponse {
  pub errors: Vec<CloudflareError>,
  // pub messages: Vec<CloudflareMessage>,
  // pub success: bool,
}
