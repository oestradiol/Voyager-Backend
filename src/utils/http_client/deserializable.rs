use serde_json::Value;

#[derive(Debug)]
pub enum Deserializable<T: for<'de> serde::Deserialize<'de>> {
  Value(Value),
  Data(T),
}
impl<T: for<'de> serde::Deserialize<'de>> Deserializable<T> {
  pub(crate) fn data(self) -> Option<T> {
    match self {
      Self::Data(data) => Some(data),
      Self::Value(_) => None,
    }
  }
}
