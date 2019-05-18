use super::super::address::Address;
use super::super::serialization::rawend::RawEnd;
use super::super::serialization::bits::Bits;
use super::connectiontype::ConnectionType;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde;
use serde::de::{Deserialize, Deserializer};
use crate::networking::payloads::payload::Ipv8Payload;

#[derive(Debug, PartialEq)]
struct IntroductionRequestPayload {
  /// is the address of the receiver.  Effectively this should be the
  /// wan address that others can use to contact the receiver.
  destination_address: Address,
  /// is the lan address of the sender.  Nodes in the same LAN
  /// should use this address to communicate.
  source_lan_address: Address,
  /// is the wan address of the sender.  Nodes not in the same
  /// LAN should use this address to communicate.
  source_wan_address: Address,
  /// When True the receiver will introduce the sender to a new
  /// node. This introduction will be facilitated by the receiver sending a puncture-request
  /// to the new node.
  advice: bool,
  // self.identifier = identifier % 65536
  // self.extra_bytes = extra_bytes
  /// indicates the connection type that the message creator has.
  connection_type: ConnectionType,

  /// is a number that must be given in the associated introduction-response.  This
  /// number allows to distinguish between multiple introduction-response messages.
  /// NOTE: u16 is the max value given by the py-ipv8 implementation
  /// (https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/payload.py#L74)
  identifier: u16,

  /// is a string that can be used to piggyback extra information.
  extra_bytes: RawEnd,
}

impl Ipv8Payload for IntroductionRequestPayload{
  /// this function is necessary for any payload which has a rawend final field. It is called to set this field. (like a setter.)
  fn set_rawend(&mut self, bytes:RawEnd){
    self.extra_bytes = bytes;
  }
}

/// makes the IntroductionRequestPayload serializable.
/// This is less than trivial as there is no 1:1 mapping between the serialized data and the payload struct.
/// Some struct fields are combined into one byte to form the serialized data.
impl Serialize for IntroductionRequestPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer{
        let conntype = self.connection_type.encode();

        let mut state = serializer.serialize_struct("IntroductionRequestPayload", 6)?;
        state.serialize_field("destination_address", &self.destination_address)?;
        state.serialize_field("source_lan_address", &self.source_lan_address)?;
        state.serialize_field("source_wan_address", &self.source_wan_address)?;
        state.serialize_field("advice", &Bits::from_bools((conntype.0, conntype.1, false, false, false, false, false, self.advice)))?;
        state.serialize_field("identifier", &self.identifier)?;
        state.serialize_field("extra_bytes", &self.extra_bytes)?;

        state.end()
    }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// this is the actual pattern of an introductionRequestPayload.
/// Used for deserializing. This is again needed because there is no 1:1 mapping between the
/// serialized data and the payload struct. This is the intermediate representation.
struct IntroductionRequestPayloadPattern(Address,Address,Address,Bits,u16);

impl<'de> Deserialize<'de> for IntroductionRequestPayload{
  /// deserializes an IntroductionRequestPayload
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where D: Deserializer<'de>,{
    // first deserialize it to a temporary struct which litterally represents the packer
    let temppayload = IntroductionRequestPayloadPattern::deserialize(deserializer);

    //now build the struct for real
    match temppayload{
      Ok(i) => Ok(IntroductionRequestPayload {
        destination_address: i.0,
        source_lan_address: i.1,
        source_wan_address: i.2,
        advice: i.3.bit7,
        connection_type: ConnectionType::decode((i.3.bit0, i.3.bit1)),
        identifier: i.4,
        extra_bytes: RawEnd(vec![]), //empty for now but will be set by deserialize
      }),
      Err(i) => Err(i) // on error just forward the error
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::net::Ipv4Addr;
  use crate::networking::serialization::{deserialize, serialize};

  #[test]
  fn integration_test_creation() {
    let i = IntroductionRequestPayload {
      destination_address: Address {
        address: Ipv4Addr::new(127, 0, 0, 1),
        port: 8000,
      },
      source_lan_address: Address {
        address: Ipv4Addr::new(42, 42, 42, 42),
        port: 8000,
      },
      source_wan_address: Address {
        address: Ipv4Addr::new(255, 255, 255, 0),
        port: 8000,
      },
      advice: true,
      connection_type: ConnectionType::decode((true, true)),
      identifier: 42,
      extra_bytes: RawEnd(vec![43, 44]),
    };

    let serialized = serialize(&i).unwrap();
    assert_eq!(
      serialized,
      vec![
          127, 0, 0, 1,64, 31, 42, 42, 42, 42,64, 31, 255, 255, 255, 0 ,64, 31, 131, 42, 0, 43,44
        ]
    );

    assert_eq!(i,deserialize(
      &serialized
    ).unwrap());
  }
}
