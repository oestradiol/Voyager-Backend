use serde_json::Value;

pub enum Deserializable<T: for<'de> serde::Deserialize<'de>> {
  Value(Value),
  Data(T),
}
impl<T: for<'de> serde::Deserialize<'de>> Deserializable<T> {
  pub(crate) fn data(self) -> Option<T> {
    match self {
      Deserializable::Data(data) => Some(data),
      Deserializable::Value(_) => None,
    }
  }
}
