use serde::{Serialize,Deserialize};
use crate::networking::payloads::Ipv8Payload;
use crate::networking::serialization::varlen::VarLen16;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BinMemberAuthenticationPayload {
  /// The global time in the system. Uses a lamport clock system.
  /// Ipv8 stores global time values using, at most, 64 bits.
  /// Therefore there is a finite number of global time values available.
  /// To avoid malicious peers from quickly pushing the global time value to the point where none are left,
  /// peers will only accept messages with a global time that is within a locally evaluated limit.
  /// This limit is set to the median of the neighbors’ global time values plus a predefined margin.
  /// https://en.wikipedia.org/wiki/Lamport_timestamps
  /// (from dispersy docs. TODO: still up to date?)
  ///
  /// TODO: has to change to a PublicKey binary representation object. The serializer should convert this to a varlen16 while serializing like in IntroductionRequestPayload.
  pub public_key_bin: VarLen16,
}

impl Ipv8Payload for BinMemberAuthenticationPayload {
  // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::networking::serialization::Packet;

  #[test]
  fn integration_test_creation() {
    let i = BinMemberAuthenticationPayload {
      public_key_bin: VarLen16(vec![121, 101, 101, 116,])
    };

    assert_eq!(
      Packet::serialize(&i).unwrap(),
      Packet(vec![0,4,121,101,101,116])
    );
    assert_eq!(i,Packet::serialize(&i).unwrap().deserialize().unwrap());
  }
}
