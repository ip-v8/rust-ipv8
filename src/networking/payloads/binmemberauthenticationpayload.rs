use serde::{Serialize,Deserialize};
use crate::networking::payloads::Ipv8Payload;
use crate::networking::serialization::varlen::VarLen16;

/// This struct represents the public key in a message.
/// This is important because with this key the signature (at the end of a packet)
/// can be verified.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BinMemberAuthenticationPayload {
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
