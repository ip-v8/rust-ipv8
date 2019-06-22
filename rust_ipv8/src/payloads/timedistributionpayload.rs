//! Send as a part of most [Packets](crate::serialization::Packet) to drive the [Lamport clock](https://en.wikipedia.org/wiki/Lamport_timestamps)
use crate::payloads::Ipv8Payload;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// The global time in the system. Uses a lamport clock system.
/// Ipv8 stores global time values using, at most, 64 bits.
/// Therefore there is a finite number of global time values available.
/// To avoid malicious peers from quickly pushing the global time value to the point where none are left,
/// peers will only accept messages with a global time that is within a locally evaluated limit.
/// This limit is set to the median of the neighbors’ global time values plus a predefined margin.
/// https://en.wikipedia.org/wiki/Lamport_timestamps
/// (from dispersy docs. TODO: still up to date?)
pub struct TimeDistributionPayload {
    /// The actual time represented as a u64
    pub global_time: u64,
}

impl Ipv8Payload for TimeDistributionPayload {
    // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialization::Packet;

    #[test]
    fn integration_test_creation() {
        let i = TimeDistributionPayload { global_time: 42u64 };

        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();

        assert_eq!(
            packet,
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0,
                0, 0, 0, 42
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
