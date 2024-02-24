use serde::Serialize;
use tracing::{event, Level};

pub fn to_json_str(obj: &(impl Serialize + std::fmt::Debug)) -> Option<String> {
  match serde_json::to_string(obj) {
    Ok(jsonstr) => Some(jsonstr),
    Err(err) => {
      event!(Level::ERROR, "Failed to convert {:?} to json string", obj);
      None
    }
  }
}
