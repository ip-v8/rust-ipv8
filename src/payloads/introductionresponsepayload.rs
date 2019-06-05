use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde;
use serde::de::{Deserialize, Deserializer};
use crate::payloads::Ipv8Payload;
use crate::networking::address::Address;
use crate::payloads::connectiontype::ConnectionType;
use crate::serialization::rawend::RawEnd;
use crate::serialization::bits::Bits;

//TODO: why the fuck is tunnel not (de)serialized
#[derive(Debug, PartialEq)]
pub struct IntroductionResponsePayload {
  /// is the address of the receiver.  Effectively this should be the
  /// wan address that others can use to contact the receiver.
  pub destination_address: Address,
  /// is the lan address of the sender.  Nodes in the same LAN
  /// should use this address to communicate.
  pub source_lan_address: Address,
  /// is the wan address of the sender.  Nodes not in the same
  /// LAN should use this address to communicate.
  pub source_wan_address: Address,
  /// is the lan address of the node that the sender
  /// advises the receiver to contact.  This address is zero when the associated request did
  /// not want advice.
  pub lan_introduction_address: Address,
  /// is the wan address of the node that the sender
  /// advises the receiver to contact.  This address is zero when the associated request did
  ///  not want advice.
  pub wan_introduction_address: Address,
  /// When True the receiver will introduce the sender to a new
  /// node. This introduction will be facilitated by the receiver sending a puncture-request
  /// to the new node.
  pub tunnel: bool,
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

impl Ipv8Payload for IntroductionResponsePayload{
  // doesnt have anything but needed for the default implementation (as of right now)
}

/// makes the IntroductionResponsePayload serializable.
/// This is less than trivial as there is no 1:1 mapping between the serialized data and the payload struct.
/// Some struct fields are combined into one byte to form the serialized data.
impl Serialize for IntroductionResponsePayload {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer{
    let conntype = self.connection_type.encode();

    //TODO: Serialize as tuple
    let mut state = serializer.serialize_struct("IntroductionResponsePayload", 6)?;
    state.serialize_field("destination_address", &self.destination_address)?;
    state.serialize_field("source_lan_address", &self.source_lan_address)?;
    state.serialize_field("source_wan_address", &self.source_wan_address)?;
    state.serialize_field("lan_introduction_address", &self.lan_introduction_address)?;
    state.serialize_field("wan_introduction_address", &self.wan_introduction_address)?;
    // the False values here correspond to unused bits in the flags field, inherited from py-ipv8.
    state.serialize_field("advice", &Bits::from_bools((conntype.0, conntype.1, false, false, false, false, false, false)))?;
    state.serialize_field("identifier", &self.identifier)?;
    state.serialize_field("extra_bytes", &self.extra_bytes)?;

    state.end()
  }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// this is the actual pattern of an IntroductionResponsePayload.
/// Used for deserializing. This is again needed because there is no 1:1 mapping between the
/// serialized data and the payload struct. This is the intermediate representation.
struct IntroductionResponsePayloadPattern(Address,Address,Address,Address,Address,Bits,u16,RawEnd);

impl<'de> Deserialize<'de> for IntroductionResponsePayload{
  /// deserializes an IntroductionResponsePayload
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,{
    // first deserialize it to a temporary struct which literally represents the packer
    let payload_temporary = IntroductionResponsePayloadPattern::deserialize(deserializer)?;

    // now build the struct for real
    Ok(IntroductionResponsePayload {
      destination_address: payload_temporary.0,
      source_lan_address: payload_temporary.1,
      source_wan_address: payload_temporary.2,
      lan_introduction_address: payload_temporary.3,
      wan_introduction_address: payload_temporary.4,
      tunnel: false,
      connection_type: ConnectionType::decode((payload_temporary.5.bit0, payload_temporary.5.bit1)),
      identifier: payload_temporary.6,
      extra_bytes: payload_temporary.7,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::net::Ipv4Addr;
  use crate::serialization::Packet;

  #[test]
  fn integration_test_creation() {
    let i = IntroductionResponsePayload {
      destination_address: Address {
        address: Ipv4Addr::new(127, 0, 0, 1),
        port: 8000,
      },
      source_lan_address: Address {
        address: Ipv4Addr::new(42, 42, 42, 42),
        port: 8001,
      },
      source_wan_address: Address {
        address: Ipv4Addr::new(255, 255, 255, 0),
        port: 8002,
      },
      lan_introduction_address: Address {
        address: Ipv4Addr::new(43, 43, 43, 43),
        port: 8003,
      },
      wan_introduction_address: Address {
        address: Ipv4Addr::new(4, 44, 44, 44),
        port: 8004,
      },
      tunnel: true,
      connection_type: ConnectionType::decode((true, true)),
      identifier: 42,
      extra_bytes: RawEnd(vec![43, 44]),
    };

    let mut packet = Packet::new(create_test_header!()).unwrap();
    packet.add(&i).unwrap();
    assert_eq!(packet,
      Packet(vec![
        0,42,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,42,
        127, 0, 0, 1,31, 64, 42, 42, 42, 42, 31, 65, 255, 255, 255, 0, 31 ,66, 43, 43, 43, 43, 31, 67, 4, 44, 44, 44, 31, 68, 3, 0, 42, 43, 44
      ])
    );

    assert_eq!(IntroductionResponsePayload {
      destination_address: Address {
        address: Ipv4Addr::new(127, 0, 0, 1),
        port: 8000,
      },
      source_lan_address: Address {
        address: Ipv4Addr::new(42, 42, 42, 42),
        port: 8001,
      },
      source_wan_address: Address {
        address: Ipv4Addr::new(255, 255, 255, 0),
        port: 8002,
      },
      lan_introduction_address: Address {
        address: Ipv4Addr::new(43, 43, 43, 43),
        port: 8003,
      },
      wan_introduction_address: Address {
        address: Ipv4Addr::new(4, 44, 44, 44),
        port: 8004,
      },
      tunnel: false, // tunnel should have changed from true to false as it is always false after deserialization
      connection_type: ConnectionType::decode((true, true)),
      identifier: 42,
      extra_bytes: RawEnd(vec![43, 44]),
    },packet.start_deserialize().skip_header().unwrap().next_payload().unwrap());
  }
}
