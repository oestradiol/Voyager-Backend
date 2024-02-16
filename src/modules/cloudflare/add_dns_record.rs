use serde::Deserialize;
use crate::modules::cloudflare::cloudflare_responses::{CloudflareError, CloudflareMessage};

#[derive(Debug, Deserialize)]
pub struct AddDnsRecordSuccess {
  pub errors: Vec<CloudflareError>,
  pub messages: Vec<CloudflareMessage>,
  pub result: AddDnsRecordData,
}

#[derive(Debug, Deserialize)]
pub struct AddDnsRecordData {
  pub content: String,
  pub name: String,
  pub proxied: bool,
  #[serde(rename = "type")]
  pub r#type: String,
  pub comment: String,
  pub id: String,
  pub locked: bool,
  pub meta: Meta,
  pub modified_on: String,
  pub proxiable: bool,
  pub tags: Vec<String>,
  pub ttl: i64,
  pub zone_id: String,
  pub zone_name: String,
}
#[derive(Debug, Deserialize)]
pub struct Meta {
  pub auto_added: bool,
  pub source: String
}

#[derive(Debug, Deserialize)]
pub struct AddDnsRecordFailure {
  pub messages: Vec<CloudflareMessage>,
  pub errors: Vec<CloudflareError>,
}
