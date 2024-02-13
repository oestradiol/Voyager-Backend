use reqwest::{Client, Method, Response};
use reqwest::header::HeaderMap;
use serde::Serialize;
use url::Url;
use crate::Error;

pub struct ClientWrapper {
  pub(crate) client: Client,
  pub(crate) uri: Url,
  pub(crate) headers: HeaderMap,
}
impl ClientWrapper {
  pub(crate) fn set_new_headers(&mut self, headers: HeaderMap) {
    let mut old_headers = self.headers.clone();
    old_headers.extend(headers);
    self.headers = old_headers;
  }

  pub(crate) async fn request<T: Serialize + Sized>(&self, method: Method, route: &str, body: Option<&T>) -> Result<Response, Error> {
    let uri = self.uri.join(route).map_err(Error::from)?;

    let client = self.client
      .request(method, uri)
      .headers(self.headers.clone());

    let req = match body {
      Some(body) => client.json(body),
      None => client
    };

    req.send().await.map_err(Error::from)
  }
}
