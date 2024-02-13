use serde::Serialize;
use crate::utils::http_client::ensure_success::EnsureSuccess;

#[derive(Debug, Serialize)]
pub struct DnsRecord {
  pub(crate) content: String,
  pub(crate) name: String,
  pub(crate) proxied: bool,
  #[serde(rename = "type")]
  pub(crate) record_type: String,
  pub(crate) ttl: u32,
  pub(crate) comment: String,
}
