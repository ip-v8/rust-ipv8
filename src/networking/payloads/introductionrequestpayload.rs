use super::super::address::Address;
use super::super::serialization::rawend::RawEnd;
use super::super::serialization::bits::Bits;
use super::connectiontype::ConnectionType;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde;
use serde::de::{Deserialize, Deserializer};
use crate::networking::payloads::Ipv8Payload;

#[derive(Debug, PartialEq)]
pub struct IntroductionRequestPayload {
  /// is the address of the receiver.  Effectively this should be the
  /// wan address that others can use to contact the receiver.
  pub destination_address: Address,
  /// is the lan address of the sender.  Nodes in the same LAN
  /// should use this address to communicate.
  pub source_lan_address: Address,
  /// is the wan address of the sender.  Nodes not in the same
  /// LAN should use this address to communicate.
  pub source_wan_address: Address,
  /// When True the receiver will introduce the sender to a new
  /// node. This introduction will be facilitated by the receiver sending a puncture-request
  /// to the new node.
  pub advice: bool,
  // self.identifier = identifier % 65536
  // self.extra_bytes = extra_bytes
  /// indicates the connection type that the message creator has.
  pub connection_type: ConnectionType,

  /// is a number that must be given in the associated introduction-response.  This
  /// number allows to distinguish between multiple introduction-response messages.
  /// NOTE: u16 is the max value given by the py-ipv8 implementation
  /// (https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/payload.py#L74)
  pub identifier: u16,

  /// is a string that can be used to piggyback extra information.
  pub extra_bytes: RawEnd,
}

impl Ipv8Payload for IntroductionRequestPayload{
  // doesnt have anything but needed for the default implementation (as of right now)
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
        // the False values here correspond to unused bits in the flags field, inherited from py-ipv8.
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
struct IntroductionRequestPayloadPattern(Address,Address,Address,Bits,u16,RawEnd);

impl<'de> Deserialize<'de> for IntroductionRequestPayload{
  /// deserializes an IntroductionRequestPayload
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where D: Deserializer<'de>,{
    // first deserialize it to a temporary struct which literally represents the packer
    let payload_temporary = IntroductionRequestPayloadPattern::deserialize(deserializer)?;

    // now build the struct for real
    Ok(IntroductionRequestPayload {
      destination_address: payload_temporary.0,
      source_lan_address: payload_temporary.1,
      source_wan_address: payload_temporary.2,
      advice: payload_temporary.3.bit7,
      connection_type: ConnectionType::decode((payload_temporary.3.bit0, payload_temporary.3.bit1)),
      identifier: payload_temporary.4,
      extra_bytes: payload_temporary.5,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::net::Ipv4Addr;
  use crate::networking::serialization::Packet;
  use crate::networking::serialization::header::{TEST_HEADER, DefaultHeader};

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

    let mut packet = Packet::new(TEST_HEADER).unwrap();
    packet.add(&i).unwrap();
    assert_eq!(
      packet,
      Packet(vec![
        0,42,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,42,
        127, 0, 0, 1,31, 64, 42, 42, 42, 42, 31, 64, 255, 255, 255, 0, 31 ,64, 131, 0, 42, 43,44
      ])
    );

    assert_eq!(i,packet.start_deserialize().skip_header::<DefaultHeader>().next().unwrap());
  }
}
