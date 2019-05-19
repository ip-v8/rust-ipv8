use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde;

/// Datatype representing the raw bytes at the end of an ipv8 payload where the length shouldn't be prefixed.
#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct RawEnd (
  #[serde(skip_deserializing)]
  pub Vec<u8>
);

impl Serialize for RawEnd {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer{
    let mut state = serializer.serialize_struct("RawEnd", self.0.len())?;
    for i in &self.0{
      state.serialize_field("value", &i)?;
    }
    state.end()
  }
}
