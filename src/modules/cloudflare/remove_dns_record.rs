use serde_json::Value;
use tracing::{event, Level};

use crate::configs::environment::{CLOUDFLARE_ZONE, DEVELOPMENT};
use crate::modules::cloudflare::CLOUDFLARE_CLIENT;
use crate::modules::cloudflare::types::cloudflare_responses::CloudflareError;
use crate::modules::cloudflare::types::delete_dns_record::{Failure, Success};
use crate::utils::http_client::ensure_success::EnsureSuccess;

async fn remove_dns_record(dns_record: &str) -> Result<(), Vec<CloudflareError>> {
  if *DEVELOPMENT {
    return Ok(());
  }

  event!(
    Level::INFO,
    "Removing DNS record from Cloudflare: {}", dns_record,
  );

  let route = format!(
    "zones/{}/dns_records/{}",
    CLOUDFLARE_ZONE.clone(), dns_record
  );
  let (is_success, response, status) = CLOUDFLARE_CLIENT
    .write().await
    .delete::<Value>(route.as_str(), Some(&dns_record)).await
    .ensure_success(false);
  if !is_success {
    event!(Level::ERROR, "Failed to send request to Add DNS Record with Cloudflare.");
    return Err(vec![]);
  }
  let response = response.unwrap().data().unwrap();
  let status = status.unwrap();

  event!(Level::DEBUG, "Request sent to Cloudflare");

  let json = serde_json::from_value::<Success>(response.clone());
  if let Ok(success) = json {
    let id = success.result.id;
    event!(Level::DEBUG, "Cloudflare request was successful with id: {}", id);
    Ok(())
  } else {
    let failure = serde_json::from_value::<Failure>(response);
    let failure = match failure {
      Ok(failure) => failure,
      Err(err) => {
        event!(Level::ERROR, "Failed to deserialize failed response for Cloudflare. Status was: {}. Error: {}", status, err);
        return Err(vec![]);
      }
    };

    event!(Level::DEBUG, "Request failed with status {} and errors: {:?}", status, failure.errors);
    Err(failure.errors)
  }
}
