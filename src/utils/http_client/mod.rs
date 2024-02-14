mod client_wrapper;
pub mod deserializable;
pub mod ensure_success;
mod generate_methods;

use std::sync::Arc;
use reqwest::{StatusCode};
use serde_json::{Value};
use reqwest::{Client, Method};
use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT, HeaderMap};
use serde::Serialize;
use tokio::sync::{RwLock};
use tracing::{event, Level};
use url::Url;
use crate::{Error, generate_methods};
use crate::utils::http_client::client_wrapper::ClientWrapper;
use crate::utils::http_client::deserializable::Deserializable;
use paste::paste;

pub type Response<T> = (Option<Deserializable<T>>, StatusCode);

pub struct HTTPClient {
  client: Arc<RwLock<ClientWrapper>>,
}
impl HTTPClient {
  fn get_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(USER_AGENT, "reqwest".parse().unwrap());
    headers
  }

  pub fn new(uri: &str, headers: Option<HeaderMap>) -> Result<Self, Error> {
    let uri = Url::parse(uri).map_err(Error::from)?;

    let mut default_headers = Self::get_default_headers();
    let headers = match headers {
      Some(headers) => {
        default_headers.extend(headers);
        default_headers
      },
      None => default_headers
    };

    Ok(Self {
      client: Arc::new(
        RwLock::new(
          ClientWrapper {
            client: Client::new(),
            uri,
            headers,
          }
        )
      )
    })
  }

  async fn get_client(&self, reset_headers: bool) -> Arc<RwLock<ClientWrapper>> {
    if reset_headers {
      self.client.write().await.set_new_headers(Self::get_default_headers());
    }

    self.client.clone()
  }

  async fn act_internal(&mut self, method: Method, route: &str, body: Option<&(impl Serialize + Send + Sync)>)
                        -> Result<reqwest::Response, Error> {

    let action = |client: Arc<RwLock<ClientWrapper>>, method: Method| async move {
      client.write().await.request(method, route, body).await
    };

    let client = self.get_client(false);

    let response = action(client.await, method).await?;

    // if response.status().as_u16() == 401 {
    //     // Unauthorized, reset client and retry
    //     client = self.get_client(true);
    //     Ok(action(client, method).await?)
    // } else {
    //     Ok(response)
    // }
    Ok(response)
  }

  async fn act<T: for<'de> serde::Deserialize<'de>>(&mut self, method: Method, route: &str, body: Option<&(impl Serialize + Send + Sync)>)
                                                    -> Result<Response<T>, Error> {

    let http_response = self.act_internal(method, route, body).await?;
    let status = http_response.status();

    // Deserialize JSON response into a dynamic serde_json::Value
    let json: String = http_response.text().await?;
    event!(Level::INFO, "HTTPClient received response: {:?}", json);

    let Ok(json) = serde_json::from_str(&json) else {
      event!(Level::ERROR, "Failed to deserialize response into JSON");
      return Ok((None, status));
    };

    match json {
      Value::Null => Ok((None, status)),
      json => {
        if !status.is_success() {
          return Ok((Some(Deserializable::Value(json)), status));
        }

        // Attempt to deserialize dynamic value into the specified type `T`
        let result = match serde_json::from_value(json.clone()) {
          Ok(deserialized) => (Some(Deserializable::Data(deserialized)), status),
          Err(e) => {
            event!(Level::ERROR, "Failed to deserialize response into specified type: {:?}", e);
            (Some(Deserializable::Value(json)), status)
          },
        };

        Ok(result)
      }
    }
  }

  generate_methods!(get, post, put, patch, delete);
}
