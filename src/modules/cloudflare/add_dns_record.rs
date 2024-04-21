use axum::http::StatusCode;
use serde_json::Value;
use tracing::{event, Level};

use crate::configs::environment::{CLOUDFLARE_ZONE, DEVELOPMENT};
use crate::modules::cloudflare::types::add_dns_record::{FailureResponse, OkResponse};
use crate::modules::cloudflare::types::dns_record::DnsRecord;
use crate::modules::cloudflare::CLOUDFLARE_CLIENT;
use crate::types::model::deployment::Mode;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::http_client::deserializable::Deserializable;
use crate::utils::http_client::ensure_success::EnsureSuccess;
use crate::utils::http_client::http_error::HttpError;
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
    comment: format!("Voyager {mode:?} for {host}"),
    ttl: 1,
  };

  let route = format!("zones/{}/dns_records", &*CLOUDFLARE_ZONE);
  let result = CLOUDFLARE_CLIENT
    .write()
    .await
    .post::<Value>(route.as_str(), Some(&dns_record))
    .await
    .ensure_success(false);

  let response: Value;
  let status: StatusCode;
  // These are already checked by the .ensure_success(false) + is_success checks above
  #[allow(clippy::unwrap_used)]
  match result {
    Ok((res, status_code)) => {
      response = res.unwrap().data().unwrap();
      status = status_code;
    },
    Err(HttpError::<Value> { response: Some(Deserializable::Value(val)), status_code: Some(status_code), .. }) => {
      response = val;
      status = status_code;
    },
    Err(e) => {
      return Err(VoyagerError::cloudflare_add_req(Box::new(e)));
    },
  }

  event!(Level::DEBUG, "Done sending request to Cloudflare");

  let json = serde_json::from_value::<OkResponse>(response.clone());
  match json {
    Ok(success) => {
      if let Some(data) = success.result {
        event!(
          Level::DEBUG,
          "Cloudflare request was successful with id: {}",
          data.id
        );
        return Ok(data.id);
      }
      Err(VoyagerError::cloudflare_get_add_id())
    },
    Err(e) => {
      event!(Level::DEBUG, "Failed to deserialize Add DNS request Success response from Cloudflare. Attempting to deserialise Failure instead. {e}");

      let failure = serde_json::from_value::<FailureResponse>(response.clone())
        .map_err(|e| VoyagerError::cloudflare_add_deserialize(Box::new(e), status, &response))?;

      Err(VoyagerError::cloudflare_add_failure(&failure, status))
    },
  }
}

impl VoyagerError {
  fn cloudflare_get_add_id() -> Self {
    Self::new(
      "Error: Cloudflare DNS Record Added ID was null!".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }

  fn cloudflare_add_req(e: Error) -> Self {
    Self::new(
      "Failed to send Add DNS request to Cloudflare".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn cloudflare_add_deserialize(e: Error, status_code: reqwest::StatusCode, response: &Value) -> Self {
    Self::new(
      format!("Failed to deserialize Add DNS request response from Cloudflare. Response was HTTP {status_code}. Value: {response}"),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn cloudflare_add_failure(failure: &FailureResponse, status_code: reqwest::StatusCode) -> Self {
    let err = failure
      .errors
      .iter()
      .fold(String::from("Cloudflare Errors:"), |acc, e| {
        format!("{acc}\n{e}")
      });

    Self::new(
      format!("Failed to Add DNS Record. Response was HTTP {status_code}. {err}"),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }
}
