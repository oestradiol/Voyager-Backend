use serde::Deserialize;
use crate::modules::cloudflare::types::cloudflare_responses::{CloudflareError, CloudflareMessage};

#[derive(Debug, Deserialize)]
pub struct DeleteDnsRecordSuccess {
  pub result: DeleteDnsRecordData,
}

#[derive(Debug, Deserialize)]
pub struct DeleteDnsRecordData {
  pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteDnsRecordFailure {
  pub errors: Vec<CloudflareError>,
  pub messages: Vec<CloudflareMessage>,
  pub success: bool,
}
