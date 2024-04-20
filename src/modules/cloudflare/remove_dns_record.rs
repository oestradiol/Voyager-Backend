use axum::http::StatusCode;
use serde_json::Value;
use tracing::{event, Level};

use crate::configs::environment::{CLOUDFLARE_ZONE, DEVELOPMENT};
use crate::modules::cloudflare::types::cloudflare_responses::CloudflareError;
use crate::modules::cloudflare::types::delete_dns_record::{Failure, Success};
use crate::modules::cloudflare::CLOUDFLARE_CLIENT;
use crate::types::other::voyager_error::VoyagerError;
use crate::utils::http_client::deserializable::Deserializable;
use crate::utils::http_client::ensure_success::EnsureSuccess;
use crate::utils::http_client::http_error::HttpError;
use crate::utils::Error;

pub async fn remove_dns_record(dns_record: &str) -> Result<(), VoyagerError> {
  if *DEVELOPMENT {
    return Ok(());
  }

  event!(
    Level::INFO,
    "Removing DNS record from Cloudflare: {}",
    dns_record,
  );

  let route = format!(
    "zones/{}/dns_records/{dns_record}",
    &*CLOUDFLARE_ZONE
  );

  let result = CLOUDFLARE_CLIENT
    .write()
    .await
    .delete::<Value>(route.as_str(), Some(&dns_record))
    .await
    .ensure_success(false);

  let mut response: Value;
  let mut status: StatusCode;
  // These are already checked by the .ensure_success(false) + is_success checks above
  #[allow(clippy::unwrap_used)]
  match result {
    Ok((res, status_code)) => {
      response = res.unwrap().data().unwrap();
      status = status_code;
    },
    Err(HttpError::<Value> { response: Some(Deserializable::Data(val)), status_code: Some(status_code), .. }) => {
      response = val;
      status = status_code;
    },
    Err(e) => {
      return Err(VoyagerError::cloudflare_remove_req(Box::new(e)));
    },
  }

  event!(Level::DEBUG, "Done sending request to Cloudflare");

  let json = serde_json::from_value::<Success>(response.clone());
  if let Ok(success) = json {
    let id = success.result.id;
    event!(
      Level::DEBUG,
      "Cloudflare request was successful with id: {}",
      id
    );
    Ok(())
  } else {
    let failure = serde_json::from_value::<Failure>(response)
      .map_err(|e| VoyagerError::cloudflare_remove_deserialize(Box::new(e), status))?;

    Err(VoyagerError::cloudflare_remove_failure(
      &failure,
      status,
    ))
  }
}

impl VoyagerError {
  fn cloudflare_remove_req(e: Error) -> Self {
    Self::new(
      "Failed to send Remove DNS request to Cloudflare".to_string(),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn cloudflare_remove_deserialize(e: Error, status_code: reqwest::StatusCode) -> Self {
    Self::new(
      format!("Failed to deserialize Remove DNS request response from Cloudflare. Response was {status_code}"),
      StatusCode::INTERNAL_SERVER_ERROR,
      Some(e),
    )
  }

  fn cloudflare_remove_failure(failure: &Failure, status_code: reqwest::StatusCode) -> Self {
    let err = failure
      .errors
      .iter()
      .fold(String::from("Cloudflare Errors:"), |acc, e| {
        format!("{acc}\n{e}")
      });

    Self::new(
      format!("Failed to Remove DNS record. Response was {status_code}. {err}"),
      StatusCode::INTERNAL_SERVER_ERROR,
      None,
    )
  }
}
