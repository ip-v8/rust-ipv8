use super::super::address::Address;
use super::packet::Packet;
use super::payload::Ipv8Payload;
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq)]
struct PunctureRequestPayload {
  /// is the lan address of the node that the sender wants us to contact.
  /// This contact attempt should punch a hole in our NAT to allow the node to
  /// connect to us.
  lan_walker_address: Address,
  /// is the lan address of the node that the sender wants us to contact.
  /// This contact attempt should punch a hole in our NAT to allow the node to
  /// connect to us.
  /// TODO differences with lan walker address as comments are the same rn.
  wan_walker_address: Address,

  /// is a number that must be given in the associated introduction-response.  This
  /// number allows to distinguish between multiple introduction-response messages.
  /// NOTE: u16 is the max value given by the py-ipv8 implementation
  /// (https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/payload.py#L74)
  identifier: u16,
}

impl Ipv8Payload for PunctureRequestPayload {
  fn pack(&self) -> Packet {
    let mut res = Packet::new();

    let lan_walker_address = self.lan_walker_address.address.octets();
    let lan_walker_port = self.lan_walker_address.port;
    let wan_walker_address = self.wan_walker_address.address.octets();
    let wan_walker_port = self.wan_walker_address.port;

    res
      .add_raw(
        vec![
          lan_walker_address[0],
          lan_walker_address[1],
          lan_walker_address[2],
          lan_walker_address[3],
        ],
        4,
      )
      .add_u16(lan_walker_port)
      .add_raw(
        vec![
          wan_walker_address[0],
          wan_walker_address[1],
          wan_walker_address[2],
          wan_walker_address[3],
        ],
        4,
      )
      .add_u16(wan_walker_port)
      .add_u16(self.identifier);
    res
  }

  fn unpack(packet: Packet) -> Self {
    let mut packetiter = packet.iter();

    let lan_walker_address = packetiter.next_raw(4).unwrap();
    let lan_walker_port = packetiter.next_u16().unwrap();

    let wan_walker_address = packetiter.next_raw(4).unwrap();
    let wan_walker_port = packetiter.next_u16().unwrap();

    let identifier = packetiter.next_u16().unwrap();

    PunctureRequestPayload {
      lan_walker_address: Address {
        address: Ipv4Addr::new(
          lan_walker_address[0],
          lan_walker_address[1],
          lan_walker_address[2],
          lan_walker_address[3],
        ),
        port: lan_walker_port,
      },
      wan_walker_address: Address {
        address: Ipv4Addr::new(
          wan_walker_address[0],
          wan_walker_address[1],
          wan_walker_address[2],
          wan_walker_address[3],
        ),
        port: wan_walker_port,
      },
      identifier,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn integration_test_creation() {
    let i = PunctureRequestPayload {
      lan_walker_address: Address {
        address: Ipv4Addr::new(127, 0, 0, 1),
        port: 8000,
      },
      wan_walker_address: Address {
        address: Ipv4Addr::new(42, 42, 42, 42),
        port: 8000,
      },
      identifier: 42,
    };

    assert_eq!(
      i.pack(),
      Packet {
        data: vec![127, 0, 0, 1, 31, 64, 42, 42, 42, 42, 31, 64, 0, 42,]
      }
    );
    assert_eq!(i, PunctureRequestPayload::unpack(i.pack()));
  }
}
