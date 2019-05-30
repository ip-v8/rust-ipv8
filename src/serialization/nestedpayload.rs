use crate::payloads::Ipv8Payload;
use crate::serialization::varlen::VarLen16;
use crate::serialization::Packet;
use serde;
use serde::de::Deserialize;
use serde::de::Deserializer;
use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;

#[derive(PartialEq, Debug)]
pub struct NestedPacket(pub Packet);

impl Ipv8Payload for NestedPacket {
    // doesnt have anything but needed for the default implementation (as of right now)
}

impl Serialize for NestedPacket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let i = VarLen16((self.0).0.to_owned());

        let mut state = serializer.serialize_struct("NestedPacket", self.0.len())?;
        state.serialize_field("payload", &i)?;
        state.end()
    }
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// this is the actual pattern of a NestedPayload.
/// Used for deserializing. This is again needed because there is no 1:1 mapping between the
/// serialized data and the payload struct. This is the intermediate representation.
struct NestedPayloadPattern(VarLen16);

impl<'de> Deserialize<'de> for NestedPacket {
    /// deserializes an IntroductionRequestPayload
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // first deserialize it to a temporary struct which literally represents the packer
        let payload_temporary: NestedPayloadPattern =
            NestedPayloadPattern::deserialize(deserializer)?;
        Ok(NestedPacket(Packet((payload_temporary.0).0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialization::header::{DefaultHeader, TEST_HEADER};
    use crate::serialization::Packet;
    use serde;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestPayload1 {
        test: NestedPacket,
    }

    impl Ipv8Payload for TestPayload1 {
        // doesnt have anything but needed for the default implementation (as of right now)
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestPayload2 {
        test: u16,
    }

    impl Ipv8Payload for TestPayload2 {
        // doesnt have anything but needed for the default implementation (as of right now)
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestPayload3 {}

    impl Ipv8Payload for TestPayload3 {
        // doesnt have anything but needed for the default implementation (as of right now)
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TestPayload4 {
        test: Vec<u8>,
    }

    impl Ipv8Payload for TestPayload4 {
        // doesnt have anything but needed for the default implementation (as of right now)
    }

    #[test]
    fn integration_test_creation() {
        let mut packet = Packet::new(TEST_HEADER).unwrap();
        packet.add(&TestPayload2 { test: 10 }).unwrap();

        let mut packet2 = Packet::new(TEST_HEADER).unwrap();
        packet2.add(&TestPayload2 { test: 10 }).unwrap();

        let i = TestPayload1 {
            test: NestedPacket(packet),
        };
        let mut newpacket = Packet::new(TEST_HEADER).unwrap();
        newpacket.add(&i);

        assert_eq!(
            i,
            newpacket
                .start_deserialize()
                .skip_header::<DefaultHeader>()
                .next_payload()
                .unwrap()
        );
    }

    #[test]
    fn test_empty() {
        let mut packet = Packet::new(TEST_HEADER).unwrap();
        packet.add(&TestPayload3 {}).unwrap();

        let mut packet2 = Packet::new(TEST_HEADER).unwrap();
        packet2.add(&TestPayload3 {}).unwrap();

        let i = TestPayload1 {
            test: NestedPacket(packet),
        };

        let mut newpacket = Packet::new(TEST_HEADER).unwrap();
        newpacket.add(&i);

        assert_eq!(
            i,
            newpacket
                .start_deserialize()
                .skip_header::<DefaultHeader>()
                .next_payload()
                .unwrap()
        );
    }

    #[test]
    fn test_too_large() {
        let tmp: Vec<u8> = vec![0; (1u32 << 17) as usize];
        let mut packet = Packet::new(TEST_HEADER).unwrap();
        packet.add(&TestPayload4 { test: tmp }).unwrap();

        let i = TestPayload1 {
            test: NestedPacket(packet),
        };

        let mut packet = Packet::new(TEST_HEADER).unwrap();
        match packet.add(&i) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
