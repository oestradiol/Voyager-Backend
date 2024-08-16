use reqwest::StatusCode;

use super::{deserializable::Deserializable, http_error::HttpError, Response};
use crate::utils::Error as OurErr;

pub trait EnsureSuccess<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> {
  fn ensure_success(
    self,
    is_nullable: bool,
  ) -> Result<(Option<Deserializable<T>>, StatusCode), HttpError<T>>;
}
impl<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> EnsureSuccess<T>
  for Result<Response<T>, OurErr>
{
  fn ensure_success(
    self,
    is_nullable: bool,
  ) -> Result<(Option<Deserializable<T>>, StatusCode), HttpError<T>> {
    let (res, status_code) = self.map_err(|e| {
      HttpError::new(
        "HTTP CLient failed to send request".to_string(),
        None,
        None,
        Some(e),
      )
    })?;

    if !status_code.is_success() {
      Err(HttpError::new(
        "Response returned error".to_string(),
        Some(status_code),
        res,
        None,
      ))
    } else if !is_nullable && res.is_none() {
      Err(HttpError::new(
        "Response body was empty on non-nullable entity".to_string(),
        Some(status_code),
        res,
        None,
      ))
    } else {
      Ok((res, status_code))
    }
  }
}
