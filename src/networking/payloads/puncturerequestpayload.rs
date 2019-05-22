use super::super::address::Address;
use super::payload::Ipv8Payload;
use serde::{Deserialize,Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PunctureRequestPayload {
  /// is the lan address of the node that the sender wants us to contact.
  /// This contact attempt should punch a hole in our NAT to allow the node to
  /// connect to us.
  pub lan_walker_address: Address,
  /// is the lan address of the node that the sender wants us to contact.
  /// This contact attempt should punch a hole in our NAT to allow the node to
  /// connect to us.
  /// TODO differences with lan walker address as comments are the same rn.
  pub wan_walker_address: Address,

  /// is a number that must be given in the associated introduction-response.  This
  /// number allows to distinguish between multiple introduction-response messages.
  /// NOTE: u16 is the max value given by the py-ipv8 implementation
  /// (https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/payload.py#L74)
  pub identifier: u16,
}

impl Ipv8Payload for PunctureRequestPayload {
  // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::networking::serialization::Packet;
  use std::net::Ipv4Addr;

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
      Packet::serialize(&i).unwrap(),
      Packet(vec![127, 0, 0, 1, 31, 64, 42, 42, 42, 42, 31, 64, 0, 42, ])
    );
    assert_eq!(i,Packet::serialize(&i).unwrap().deserialize().unwrap());
  }
}
