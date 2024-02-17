use serde_json::Value;
use tracing::{event, Level};

use crate::configs::environment::{CLOUDFLARE_ZONE, DEVELOPMENT};
use crate::modules::cloudflare::types::cloudflare_responses::CloudflareError;
use crate::modules::cloudflare::types::delete_dns_record::{Failure, Success};
use crate::modules::cloudflare::CLOUDFLARE_CLIENT;
use crate::utils::http_client::ensure_success::EnsureSuccess;

pub async fn remove_dns_record(dns_record: &str) -> Option<()> {
  if *DEVELOPMENT {
    return Some(());
  }

  event!(
    Level::INFO,
    "Removing DNS record from Cloudflare: {}",
    dns_record,
  );

  let route = format!(
    "zones/{}/dns_records/{}",
    CLOUDFLARE_ZONE.clone(),
    dns_record
  );
  let (is_success, response, status) = CLOUDFLARE_CLIENT
    .write()
    .await
    .delete::<Value>(route.as_str(), Some(&dns_record))
    .await
    .ensure_success(false);
  if !is_success {
    event!(
      Level::ERROR,
      "Failed to send request to Add DNS Record with Cloudflare."
    );
    return None;
  }
  // These are already checked by the .ensure_success(false) + is_success checks above
  #[allow(clippy::unwrap_used)]
  let response = response.unwrap().data().unwrap();
  #[allow(clippy::unwrap_used)]
  let status = status.unwrap();

  event!(Level::DEBUG, "Request sent to Cloudflare");

  let json = serde_json::from_value::<Success>(response.clone());
  if let Ok(success) = json {
    let id = success.result.id;
    event!(
      Level::DEBUG,
      "Cloudflare request was successful with id: {}",
      id
    );
    Some(())
  } else {
    let failure = serde_json::from_value::<Failure>(response);
    let failure = match failure {
      Ok(failure) => failure,
      Err(err) => {
        event!(
          Level::ERROR,
          "Failed to deserialize failed response for Cloudflare. Status was: {}. Error: {}",
          status,
          err
        );
        return None;
      }
    };

    let err = failure
      .errors
      .iter()
      .fold(String::from("Cloudflare Errors:"), |acc, e| {
        format!("{acc}\n{e}")
      });
    event!(
      Level::ERROR,
      "Request failed with status {status}. Failed to remove DNS record. {}",
      err
    );

    None
  }
}
