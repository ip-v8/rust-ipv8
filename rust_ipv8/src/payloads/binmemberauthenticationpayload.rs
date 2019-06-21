use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde::ser::SerializeTuple;

use crate::crypto::signature::Ed25519PublicKey;
use crate::payloads::Ipv8Payload;
use crate::serialization::varlen::VarLen16;

/// This struct represents the public key in a message.
/// This is important because with this key the signature (at the end of a packet)
/// can be verified.
#[derive(Debug, PartialEq)]
pub struct BinMemberAuthenticationPayload {
    /// TODO: has to change to a PublicKey binary representation object. The serializer should convert this to a varlen16 while serializing like in IntroductionRequestPayload.
    pub public_key_bin: Ed25519PublicKey,
    pub encryption_key_bin: [u8; 32],
}

/// makes the BinMemberAuthenticationPayload serializable.
/// This is less than trivial as there is no 1:1 mapping between the serialized data and the payload struct.
/// Some struct fields are combined into one byte to form the serialized data.
impl Serialize for BinMemberAuthenticationPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // + 10 for the LibNaCLPK: header
        let length = self.public_key_bin.len() + self.encryption_key_bin.len() + 10;

        // + 2 for the lengthefix
        let mut state = serializer.serialize_tuple(length + 2)?;
        state.serialize_element(&(length as u16))?;

        //"LibNaCLPK:" in bytes
        for i in [76u8, 105u8, 98u8, 78u8, 97u8, 67u8, 76u8, 80u8, 75u8, 58u8].iter() {
            state.serialize_element(&i)?;
        }

        for i in self.encryption_key_bin.iter() {
            state.serialize_element(&i)?;
        }

        for i in self.public_key_bin.iter() {
            state.serialize_element(&i)?;
        }

        state.end()
    }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// this is the actual pattern of an BinMemberAuthenticationPayload.
/// Used for deserializing. This is again needed because there is no 1:1 mapping between the
/// serialized data and the payload struct. This is the intermediate representation.
struct BinMemberAuthenticationPayloadPattern(VarLen16);

impl<'de> Deserialize<'de> for BinMemberAuthenticationPayload {
    /// deserializes an IntroductionRequestPayload
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // first deserialize it to a temporary struct which literally represents the packer
        let payload_temporary = BinMemberAuthenticationPayloadPattern::deserialize(deserializer)?;

        // payload_temporary.0.0 is the zeroth element in IntroductionRequestPayloadPattern which has a varlen which has a vector as zeroth element
        let contents = &*(payload_temporary.0).0;

        //"LibNaCLPK:" in bytes
        if contents[0..10] != [76u8, 105u8, 98u8, 78u8, 97u8, 67u8, 76u8, 80u8, 75u8, 58u8] {
            return Err(serde::de::Error::custom(
                "Received BinMemberAuthenticationPayload without LibNaclPK: prefix",
            ));
        }

        let encryption_key_bin = *zerocopy::LayoutVerified::<_, [u8; 32]>::new(&contents[10..42])
            .ok_or_else(|| {
            serde::de::Error::custom("Received BinMemberAuthenticationPayload had an invalid size")
        })?;
        let public_key_bin = *zerocopy::LayoutVerified::<_, [u8; 32]>::new(&contents[42..74])
            .ok_or_else(|| {
                serde::de::Error::custom(
                    "Received BinMemberAuthenticationPayload had an invalid size",
                )
            })?;

        // now build the struct for real
        Ok(BinMemberAuthenticationPayload {
            encryption_key_bin,
            public_key_bin,
        })
    }
}

impl Ipv8Payload for BinMemberAuthenticationPayload {
    // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
    use crate::serialization::Packet;

    use super::*;

    #[test]
    fn integration_test_creation() {
        let i = BinMemberAuthenticationPayload {
            encryption_key_bin: [
                3, 161, 7, 191, 243, 206, 16, 190, 29, 112, 221, 24, 231, 75, 192, 153, 103, 228,
                214, 48, 155, 165, 13, 95, 29, 220, 134, 100, 18, 85, 49, 184,
            ],
            public_key_bin: [
                3, 161, 7, 191, 243, 206, 16, 190, 29, 112, 221, 24, 231, 75, 192, 153, 103, 228,
                214, 48, 155, 165, 13, 95, 29, 220, 134, 100, 18, 85, 49, 184,
            ],
        };
        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();
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
