use super::super::address::{IPAddress, IPVersion};
use super::packet::Packet;
use super::payload::Ipv8Payload;
use std::string::String;

enum ConnectionType {
  UNKNOWN,
  PUBLIC,
  SYMMETRICNAT,
}

struct IntroductionRequestPayload {
  /// is the address of the receiver.  Effectively this should be the
  /// wan address that others can use to contact the receiver.
  destination_address: IPAddress,
  /// is the lan address of the sender.  Nodes in the same LAN
  /// should use this address to communicate.
  source_lan_address: IPAddress,
  /// is the wan address of the sender.  Nodes not in the same
  /// LAN should use this address to communicate.
  source_wan_address: IPAddress,
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
  extra_bytes: String,
}

impl Ipv8Payload for IntroductionRequestPayload {
  fn pack(&self) -> Packet {
    return Packet::from(&[1, 2]);
  }

  fn unpack(packet: Packet) -> Self {
  //   // packet format copied from:
  //   // https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/payload.py#L39
  //   packet.unpack(vec![
  //     CHAR, CHAR, CHAR, CHAR, U16,
  //     CHAR, CHAR, CHAR, CHAR, U16,
  //     CHAR, CHAR, CHAR, CHAR, U16,
  //     BITS, // the connection type and also stores advice in the last bit
  //     U16,
  //     STRING,
  //   ]);

    IntroductionRequestPayload {
      destination_address: IPAddress {
        address: String::from("1.2.3.4"),
        port: 6421,
        version: IPVersion::IPV4,
      },
      source_lan_address: IPAddress {
        address: String::from("1.2.3.4"),
        port: 6421,
        version: IPVersion::IPV4,
      },
      source_wan_address: IPAddress {
        address: String::from("1.2.3.4"),
        port: 6421,
        version: IPVersion::IPV4,
      },
      advice: true,
      connection_type: ConnectionType::PUBLIC,
      identifier: 1,
      extra_bytes: String::from("42"),
    }
  }
}
