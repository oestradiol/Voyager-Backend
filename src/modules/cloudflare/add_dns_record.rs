use axum::http::StatusCode;
use serde_json::Value;
use tracing::{event, Level};

use crate::configs::environment::{CLOUDFLARE_ZONE, DEVELOPMENT};
use crate::modules::cloudflare::types::add_dns_record::{Failure, Success};
use crate::modules::cloudflare::types::cloudflare_responses::CloudflareError;
use crate::modules::cloudflare::types::dns_record::DnsRecord;
use crate::modules::cloudflare::CLOUDFLARE_CLIENT;
use crate::types::model::deployment::Mode;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::http_client::ensure_success::EnsureSuccess;
use crate::utils::Error;

pub async fn add_dns_record(host: &str, ip: &str, mode: &Mode) -> Result<String, VoyagerError> {
  if *DEVELOPMENT {
    return Ok("devDnsRecord".to_string());
  }

  event!(
    Level::INFO,
    "Adding DNS record to Cloudflare for host: {}, ip: {}, mode: {:?}",
    host,
    ip,
    mode
  );

  let dns_record = DnsRecord {
    content: ip.to_string(),
    name: host.to_string(),
    proxied: true,
    record_type: "A".to_string(),
    ttl: 1,
    comment: format!("Voyager {mode:?} for {host}"),
  };

  let route = format!("zones/{}/dns_records", *CLOUDFLARE_ZONE);

  let (response, status_code) = CLOUDFLARE_CLIENT
    .write()
    .await
    .post::<Value>(route.as_str(), Some(&dns_record))
    .await
    .ensure_success(false)
    .map_err(|e| VoyagerError::cloudflare_add_req(Box::new(e)))?;
  // These are already checked by the .ensure_success(false) + is_success checks above
  #[allow(clippy::unwrap_used)]
  let response = response.unwrap().data().unwrap();

  event!(Level::DEBUG, "Request sent to Cloudflare");

  let json = serde_json::from_value::<Success>(response.clone());
  if let Ok(success) = json {
    let id = success.result.id;
    event!(
      Level::DEBUG,
      "Cloudflare request was successful with id: {}",
      id
    );
    Ok(id)
  } else {
    let failure = serde_json::from_value::<Failure>(response)
      .map_err(|e| VoyagerError::cloudflare_add_deserialize(Box::new(e), status_code))?;

    Err(VoyagerError::cloudflare_add_failure(&failure, status_code))
  }
}

impl VoyagerError {
  fn cloudflare_add_req(e: Error) -> Self {
    Self::new(
      "Failed to send Add DNS request to Cloudflare".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn cloudflare_add_deserialize(e: Error, status_code: reqwest::StatusCode) -> Self {
    Self::new(
      "Failed to deserialize Add DNS request response from Cloudflare".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn cloudflare_add_failure(failure: &Failure, status_code: reqwest::StatusCode) -> Self {
    let err = failure
      .errors
      .iter()
      .fold(String::from("Cloudflare Errors:"), |acc, e| {
        format!("{acc}\n{e}")
      });

    Self::new(
      format!("Failed to Add DNS Record. {err}"),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }
}
