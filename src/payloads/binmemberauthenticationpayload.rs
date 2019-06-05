use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, Deserializer};
use serde::{ser, de};
use serde::ser::SerializeTuple;
use crate::payloads::Ipv8Payload;
use crate::serialization::varlen::VarLen16;
use crate::crypto::keytypes::PublicKey;

/// This struct represents the public key in a message.
/// This is important because with this key the signature (at the end of a packet)
/// can be verified.
#[derive(Debug, PartialEq)]
pub struct BinMemberAuthenticationPayload {
  /// TODO: has to change to a PublicKey binary representation object. The serializer should convert this to a varlen16 while serializing like in IntroductionRequestPayload.
  pub public_key_bin: PublicKey,
}

/// makes the BinMemberAuthenticationPayload serializable.
/// This is less than trivial as there is no 1:1 mapping between the serialized data and the payload struct.
/// Some struct fields are combined into one byte to form the serialized data.
impl Serialize for BinMemberAuthenticationPayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    let v = match self.public_key_bin.to_vec() {
      Some(i) => i,
      None => return Err(ser::Error::custom("The key was malformed in a way which made it unserializable."))
    };

    let mut state = serializer.serialize_tuple(v.len() + 2)?;
    state.serialize_element(&(v.len() as u16))?;
    for i in v {
      state.serialize_element(&i)?;
    }
    state.end()
  }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// this is the actual pattern of an BinMemberAuthenticationPayload.
/// Used for deserializing. This is again needed because there is no 1:1 mapping between the
/// serialized data and the payload struct. This is the intermediate representation.
struct BinMemberAuthenticationPayloadPattern(VarLen16);

impl<'de> Deserialize<'de> for BinMemberAuthenticationPayload {
  /// deserializes an IntroductionRequestPayload
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
    // first deserialize it to a temporary struct which literally represents the packer
    let payload_temporary = BinMemberAuthenticationPayloadPattern::deserialize(deserializer)?;

    // now build the struct for real
    Ok(BinMemberAuthenticationPayload {
      // payload_temporary.0.0 is the zeroth element in IntroductionRequestPayloadPattern which has a varlen which has a vector as zeroth element
      public_key_bin: match PublicKey::from_vec((payload_temporary.0).0) {
        Some(i) => i,
        None => return Err(de::Error::custom("The key was malformed in a way which made it undeserializable."))
      }
    })
  }
}

impl Ipv8Payload for BinMemberAuthenticationPayload {
  // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::serialization::Packet;

  #[test]
  fn integration_test_creation() {
    let i = BinMemberAuthenticationPayload {
      public_key_bin: PublicKey::from_vec(vec![76,105,98,78,97,67,76,80,75,58,3,161,7,191,243,206,16,190,29,112,221,24,231,75,192,153,103,228,214,48,155,165,13,95,29,220,134,100,18,85,49,184,3,161,7,191,243,206,16,190,29,112,221,24,231,75,192,153,103,228,214,48,155,165,13,95,29,220,134,100,18,85,49,184,]).unwrap()
    };
    let mut packet = Packet::new(create_test_header!()).unwrap();
    packet.add(&i).unwrap();
    assert_eq!(i, packet.start_deserialize().skip_header().unwrap().next_payload().unwrap());
  }
}
