use super::deserializable::Deserializable;
use crate::utils::Error as OurErr;
use reqwest::StatusCode;
use std::{error::Error, fmt};
use tracing::{event, Level};

#[derive(Debug)]
pub struct HttpError<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> {
  pub message: String,
  pub status_code: Option<StatusCode>,
  pub response: Option<Deserializable<T>>,
  pub source: Option<OurErr>,
}

impl<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> fmt::Display for HttpError<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let message = self.status_code.map_or_else(
      || self.message.clone(),
      |status_code| format!("Status Code: HTTP {status_code}. {}", self.message),
    );

    let message = self.status_code.map_or_else(
      || self.message.clone(),
      |status_code| format!("Status Code: HTTP {status_code}. {}", self.message),
    );

    write!(f, "{message}")
  }
}

impl<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> Error for HttpError<T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.source.as_deref().map(|s| s as _)
  }
}

impl<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> HttpError<T> {
  pub fn new(
    message: String,
    status_code: Option<StatusCode>,
    response: Option<Deserializable<T>>,
    source: Option<OurErr>,
  ) -> Self {
    let result = Self {
      message,
      status_code,
      response,
      source,
    };

    event!(Level::ERROR, "{result}");

    result
  }
}
