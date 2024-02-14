use reqwest::StatusCode;
use tracing::{event, Level};
use crate::Error;
use crate::utils::http_client::deserializable::Deserializable;
use crate::utils::http_client::Response;

pub trait EnsureSuccess<T: for<'de> serde::Deserialize<'de>> {
  fn ensure_success(self, is_nullable: bool) -> (bool, Option<Deserializable<T>>, Option<StatusCode>);
}
impl<T: for<'de> serde::Deserialize<'de>> EnsureSuccess<T> for Result<Response<T>, Error> {
  fn ensure_success(self, is_nullable: bool) -> (bool, Option<Deserializable<T>>, Option<StatusCode>) {
    match self {
      Ok(response) => {
        let (res, status_code) = response;

        if !status_code.is_success() {
          event!(Level::ERROR, "Status Code: HTTP {status_code}. Response returned error");
          (false, res, Some(status_code))
        } else if !is_nullable && res.is_none() {
          event!(Level::ERROR, "Status Code: HTTP {status_code}. Response body was empty on non-nullable entity");
          (false, res, Some(status_code))
        } else {
          (true, res, Some(status_code))
        }
      },

      Err(e) => {
        event!(Level::ERROR, "HTTP Client failed to contact the API. Error: {e}");
        (false, None, None)
      }
    }
  }
}
