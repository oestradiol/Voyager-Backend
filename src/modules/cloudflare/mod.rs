mod add_dns_record;
mod delete_dns_record;
mod types;

pub use add_dns_record::*;
pub use delete_dns_record::*;
pub use types::*;

use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::configs::environment::CLOUDFLARE_API_TOKEN;
use crate::utils::ExpectError;
use crate::utils::http_client::HTTPClient;

lazy_static! {
  pub static ref CLOUDFLARE_CLIENT: Arc<RwLock<HTTPClient>> = {
    let mut headers = HeaderMap::new();
    #[allow(clippy::unwrap_used)] // Should never fail
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    #[allow(clippy::unwrap_used)] // Should never fail
    headers.insert(AUTHORIZATION, format!("Bearer {}", &*CLOUDFLARE_API_TOKEN).parse().unwrap());
    HTTPClient::new("https://api.cloudflare.com/client/v4/", Some(headers)).map_or_else(
      |e| panic!("Failed to create API client: {e}"),
      |k| Arc::new(RwLock::new(k)),
    )
  };
}
