mod add_dns_record;
mod delete_dns_record;
mod cloudflare_responses;

use reqwest::header::{CONTENT_TYPE, AUTHORIZATION};
use reqwest::Client;
use serde::Serialize;
use tracing::{event, Level};

use crate::configs::environment::{CLOUDFLARE_API_TOKEN, CLOUDFLARE_ZONE, DEVELOPMENT};
use crate::modules::cloudflare::add_dns_record::{AddDnsRecordFailure, AddDnsRecordSuccess};
use crate::modules::cloudflare::cloudflare_responses::CloudflareError;
use crate::modules::cloudflare::delete_dns_record::DeleteDnsRecordFailure;

#[derive(Debug, Serialize)]
struct DnsRecord {
    content: String,
    name: String,
    proxied: bool,
    #[serde(rename = "type")]
    record_type: String,
    ttl: u32,
    comment: String,
}

#[derive(Debug, Serialize)]
enum DeploymentMode {
  Preview,
  Production,
}

struct CloudflareManager {
    http_client: Client,
}

impl CloudflareManager {
  fn new() -> Self {
      CloudflareManager {
          http_client: Client::new(),
      }
  }

  async fn add_dns_record(
      &self,
      host: &str,
      ip: &str,
      mode: DeploymentMode,
  ) -> Result<String, Vec<CloudflareError>> {
    if *DEVELOPMENT {
      return Ok("devDnsRecord".to_string());
    }

    event!(
      Level::INFO,
      "Adding DNS record to cloudflare for host: {}, ip: {}, mode: {:?}",
      host, ip, mode
    );

    let dns_record = DnsRecord {
      content: ip.to_string(),
      name: host.to_string(),
      proxied: true,
      record_type: "A".to_string(),
      ttl: 1,
      comment: format!("Voyager {:?} for {}", mode, host),
    };

    let response = self
      .http_client
      .post(&format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
        CLOUDFLARE_ZONE.to_string()
      ))
      .header(CONTENT_TYPE, "application/json")
      .header(AUTHORIZATION, CLOUDFLARE_API_TOKEN.clone())
      .json(&dns_record)
      .send()
      .await;

    let response = match response {
      Ok(response) => response,
      Err(err) => {
        event!(Level::ERROR, "Failed to send request to Cloudflare: {}", err);
        return Err(vec![]);
      }
    };

    event!(Level::DEBUG, "Request sent to Cloudflare");

    let status = response.status();
    let response = response.text().await;
    let response = match response {
      Ok(response) => response,
      Err(err) => {
        event!(Level::ERROR, "Failed to deserialize response text for Cloudflare. Status was: {}. Error: {}", status, err);
        return Err(vec![]);
      }
    };

    let json = serde_json::from_str::<AddDnsRecordSuccess>(&response);

    match json {
      Ok(success) => {
        let id = success.result.id;
        event!(Level::DEBUG, "Cloudflare request was successful with id: {}", id);
        Ok(id)
      }
      Err(_) => {
        let failure = serde_json::from_str::<AddDnsRecordFailure>(&response);
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
  }

  async fn remove_dns_record(
    &self,
    dns_record: &str,
  ) -> Result<(), Vec<CloudflareError>> {
    if *DEVELOPMENT {
      return Ok(());
    }

    event!(
      Level::INFO,
      "Removing DNS record from cloudflare: {}", dns_record,
    );

    let response = self
      .http_client
      .delete(&format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        CLOUDFLARE_ZONE.clone(), dns_record
      ))
      .header(CONTENT_TYPE, "application/json")
      .header(AUTHORIZATION, CLOUDFLARE_API_TOKEN.clone())
      .send()
      .await;

    let response = match response {
      Ok(response) => response,
      Err(err) => {
        event!(Level::ERROR, "Failed to send request to Cloudflare: {}", err);
        return Err(vec![]);
      }
    };

    event!(Level::DEBUG, "Request sent to Cloudflare");
    event!(Level::DEBUG, "Response: {:?}", response);

    let status = response.status();

    if status.is_success() {
      event!(Level::DEBUG, "Cloudflare request was successful");
      return Ok(());
    }

    let errors = response.json::<DeleteDnsRecordFailure>().await;
    let errors = match errors {
      Ok(errors) => errors,
      Err(err) => {
        event!(Level::ERROR, "Failed to deserialize response for Cloudflare. Status was: {}. Error: {}", status, err);
        return Err(vec![]);
      }
    };
    let errors = errors.errors;

    event!(Level::DEBUG, "Request failed with status {} and errors: {:?}", status, errors);
    Err(errors)
  }
}
