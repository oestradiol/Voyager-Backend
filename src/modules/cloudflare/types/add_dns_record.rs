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
  // pub zone_id: String,
  // pub zone_name: String,
  // pub name: String,
  // #[serde(rename = "type")]
  // pub r#type: String,
  // pub content: String,
  // pub proxiable: bool,
  // pub proxied: bool,
  // pub ttl: i64,
  // pub locked: bool,
  // pub meta: Meta,
  // pub comment: String,
  // pub tags: Vec<String>,
  // pub created_on: String,
  // pub modified_on: String,
}
#[derive(Debug, Deserialize)]
pub struct Meta {
  // pub auto_added: bool,
  // pub managed_by_apps: bool,
  // pub managed_by_argo_tunnel: bool,
}

#[derive(Debug, Deserialize)]
pub struct FailureResponse {
  // pub messages: Vec<CloudflareMessage>,
  pub errors: Vec<CloudflareError>,
}
