
use super::super::address::Address;
use super::bits::Bits;

use super::packet::Packet;
use super::payload::Ipv8Payload;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq)]
enum ConnectionType {
  UNKNOWN,
  PUBLIC,
  SYMMETRICNAT,
}

impl ConnectionType {
  fn encode(&self) -> (bool, bool) {
    match self {
      ConnectionType::UNKNOWN => (false, false),
      ConnectionType::PUBLIC => (true, false),
      ConnectionType::SYMMETRICNAT => (true, true),
    }
  }

  fn decode(bits: (bool, bool)) -> Self {
    match bits {
      (false, false) => ConnectionType::UNKNOWN,
      (true, false) => ConnectionType::PUBLIC,
      (false, true) => ConnectionType::UNKNOWN, // not in py-ipv8 but this case is not specified and thus unknown
      (true, true) => ConnectionType::SYMMETRICNAT,
    }
  }
}

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
  extra_bytes: Vec<u8>,
}

impl Ipv8Payload for IntroductionRequestPayload {
  fn pack(&self) -> Packet {
    // return Packet::from(&[1, 2]);
    let mut res = Packet::new();

    let destination_address = self.destination_address.address.octets();
    let destination_port = self.destination_address.port;
    let source_lan_address = self.source_lan_address.address.octets();
    let source_lan_port = self.source_lan_address.port;
    let source_wan_address = self.source_wan_address.address.octets();
    let source_wan_port = self.source_wan_address.port;

    let conntype = self.connection_type.encode();
    res
      .add_raw(
        vec![
          destination_address[0],
          destination_address[1],
          destination_address[2],
          destination_address[3],
        ],
        4,
      )
      .add_u16(destination_port)
      .add_raw(
        vec![
          source_lan_address[0],
          source_lan_address[1],
          source_lan_address[2],
          source_lan_address[3],
        ],
        4,
      )
      .add_u16(source_lan_port)
      .add_raw(
        vec![
          source_wan_address[0],
          source_wan_address[1],
          source_wan_address[2],
          source_wan_address[3],
        ],
        4,
      )
      .add_u16(source_wan_port)
      .add_bits(Bits::new(
        conntype.0,
        conntype.1,
        false,
        false,
        false,
        false,
        false,
        self.advice,
      ))
      .add_u16(self.identifier)
      .add_raw_remaining(self.extra_bytes.clone());
    res
  }

  fn unpack(packet: Packet) -> Self {
    let mut packetiter = packet.iter();

    let destination_ipaddress = packetiter.next_raw(4).unwrap();
    let destination_port = packetiter.next_u16().unwrap();

    let source_lan_address = packetiter.next_raw(4).unwrap();
    let source_lan_port = packetiter.next_u16().unwrap();

    let source_wan_address = packetiter.next_raw(4).unwrap();
    let source_wan_port = packetiter.next_u16().unwrap();

    // flags are cc00000a where cc is two bits signifying the connection type and a signifies the advice
    let flags = packetiter.next_bits().unwrap();

    let identifier = packetiter.next_u16().unwrap();

    let extra_bytes = packetiter.next_raw_remaining().unwrap();

    IntroductionRequestPayload {
      destination_address: Address {
        address: Ipv4Addr::new(
          destination_ipaddress[0],
          destination_ipaddress[1],
          destination_ipaddress[2],
          destination_ipaddress[3],
        ),
        port: destination_port,
      },
      source_lan_address: Address {
        address: Ipv4Addr::new(
          source_lan_address[0],
          source_lan_address[1],
          source_lan_address[2],
          source_lan_address[3],
        ),
        port: source_lan_port,
      },
      source_wan_address: Address {
        address: Ipv4Addr::new(
          source_wan_address[0],
          source_wan_address[1],
          source_wan_address[2],
          source_wan_address[3],
        ),
        port: source_wan_port,
      },
      advice: flags.bit7,
      connection_type: ConnectionType::decode((flags.bit0, flags.bit1)),
      identifier: identifier,
      extra_bytes: extra_bytes,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

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
        address: Ipv4Addr::new(255, 255, 255, 255),
        port: 8000,
      },
      advice: true,
      connection_type: ConnectionType::decode((true, true)),
      identifier: 42,
      extra_bytes: vec![43, 44],
    };

    assert_eq!(
      i.pack(),
      Packet {
        data: vec![
          127, 0, 0, 1, 31, 64, 42, 42, 42, 42, 31, 64, 255, 255, 255, 255, 31, 64, 131, 0, 42, 43,
          44
        ]
      }
    );
    assert_eq!(i, IntroductionRequestPayload::unpack(i.pack()));
  }
}
