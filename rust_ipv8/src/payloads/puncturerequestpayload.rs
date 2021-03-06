//! Payload sent before performing [NAT puncturing](https://en.wikipedia.org/wiki/UDP_hole_punching)

use crate::payloads::Ipv8Payload;
use serde::{Deserialize, Serialize};
use crate::networking::address::Address;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// The actual payload used when requesting a NAT puncture.
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
    use crate::serialization::Packet;
    use std::net::{Ipv4Addr, SocketAddr, IpAddr};

    #[test]
    fn integration_test_creation() {
        let i = PunctureRequestPayload {
            lan_walker_address: Address(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                8000,
            )),
            wan_walker_address: Address(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(42, 42, 42, 42)),
                8000,
            )),
            identifier: 42,
        };

        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();

        assert_eq!(
            packet,
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 127, 0, 0, 1,
                31, 64, 42, 42, 42, 42, 31, 64, 0, 42,
            ])
        );
        assert_eq!(
            i,
            packet
                .start_deserialize()
                .skip_header()
                .unwrap()
                .next_payload()
                .unwrap()
        );
    }
}
