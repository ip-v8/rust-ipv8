use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};
use serde::ser::SerializeTuple;

pub trait Header {
  /// should return the serialized size of the header in bytes
  fn size() -> usize ;
  fn version(&self) -> u32;
}

#[derive(PartialEq,Debug)]
pub struct DefaultHeader{
  pub version : u16,
  pub mid_hash : [u8; 20],
  pub message_type : u8,
}

impl Header for DefaultHeader{
  fn size() -> usize{
    23
  }

  fn version(&self) -> u32{
    self.version as u32
  }
}

pub(crate) const TEST_HEADER: DefaultHeader = DefaultHeader{
  version: 42,
  mid_hash: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
  message_type: 42
};

/// makes the BinMemberAuthenticationPayload serializable.
/// This is less than trivial as there is no 1:1 mapping between the serialized data and the payload struct.
/// Some struct fields are combined into one byte to form the serialized data.
impl Serialize for DefaultHeader {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {

    // 23 is the size of the Header.
    let mut state = serializer.serialize_tuple(23)?;
    state.serialize_element(&(self.version))?;
    for i in &self.mid_hash {
      state.serialize_element(&i)?;
    }
    state.serialize_element(&(self.message_type))?;
    state.end()
  }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// this is the actual pattern of an BinMemberAuthenticationPayload.
/// Used for deserializing. This is again needed because there is no 1:1 mapping between the
/// serialized data and the payload struct. This is the intermediate representation.
struct DefaultHeaderPattern(
  u16, // version
  u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8, // mid hash (always 20*u8)
  u8 // message type
);

impl<'de> Deserialize<'de> for DefaultHeader {
  /// deserializes an IntroductionRequestPayload
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
    // first deserialize it to a temporary struct which literally represents the packer
    let payload_temporary = DefaultHeaderPattern::deserialize(deserializer)?;

    let mid_hash = [
      payload_temporary.1,payload_temporary.2,payload_temporary.3,payload_temporary.4,payload_temporary.5,payload_temporary.6,payload_temporary.7,payload_temporary.8,payload_temporary.9,payload_temporary.10,payload_temporary.11,payload_temporary.12,payload_temporary.13,payload_temporary.14,payload_temporary.15,payload_temporary.16,payload_temporary.17,payload_temporary.18,payload_temporary.19,payload_temporary.20,
    ]; // yes this is really the only way as far as we know. :((

    // now build the struct for real
    Ok(DefaultHeader {
      version: payload_temporary.0,
      mid_hash,
      message_type: payload_temporary.21,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use bincode;

  #[test]
  fn integration_test_creation() {
    let i = DefaultHeader {
      version: 42,
      mid_hash: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19],
      message_type: 42,
    };
    assert_eq!(i,bincode::config().big_endian().deserialize(
      &bincode::config().big_endian().serialize(&i).unwrap()
    ).unwrap());
  }
}

