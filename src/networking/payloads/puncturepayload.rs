use super::super::address::Address;
use super::payload::Ipv8Payload;
use serde::{Serialize,Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PuncturePayload {
  /// is the lan address of the sender.  Nodes in the same LAN
  /// should use this address to communicate.
  lan_walker_address: Address,
  /// is the wan address of the sender.  Nodes not in the same
  /// LAN should use this address to communicate.
  wan_walker_address: Address,

  /// is a number that was given in the associated introduction-request.  This
  /// number allows to distinguish between multiple introduction-response messages.
  identifier: u16,
}

impl Ipv8Payload for PuncturePayload {
  // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::networking::serialization::{serialize, deserialize};
  use std::net::Ipv4Addr;

  #[test]
  fn integration_test_creation() {
    let i = PuncturePayload {
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
      serialize(&i).unwrap(),
      vec![127, 0, 0, 1, 64, 31, 42, 42, 42, 42, 64, 31, 42, 0]
    );
    assert_eq!(i, deserialize(
      &serialize(&i).unwrap()
    ).unwrap());
  }
}
