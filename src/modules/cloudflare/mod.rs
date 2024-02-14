pub mod types;
mod add_dns_record;
mod remove_dns_record;

use std::sync::Arc;
use lazy_static::lazy_static;
use reqwest::header::{CONTENT_TYPE, AUTHORIZATION, HeaderMap};
use tokio::sync::RwLock;

use crate::configs::environment::{CLOUDFLARE_API_TOKEN};
use crate::utils::http_client::HTTPClient;

lazy_static! {
    pub static ref CLOUDFLARE_CLIENT: Arc<RwLock<HTTPClient>> = {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, CLOUDFLARE_API_TOKEN.clone().parse().unwrap());
        HTTPClient::new("https://api.cloudflare.com/client/v4", Some(headers)).map_or_else(|e| panic!("Failed to create API client: {e}"), |k| Arc::new(RwLock::new(k)))
    };
}
