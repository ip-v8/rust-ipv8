//! Module containing everything related to the Varlen data structure
use crate::payloads::Ipv8Payload;
use serde;
use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Error, Serialize, SerializeStruct, Serializer};
use std::fmt;

/// Struct representing a payload section of variable length section of a payload.
/// VarLen16 means the max length of the variable length section is 2^16 bytes
#[derive(PartialEq, Debug)]
pub struct VarLen16(pub Vec<u8>);
impl Ipv8Payload for VarLen16 {}

/// Struct representing a payload section of variable length section of a payload.
/// VarLen16 means the max length of the variable length section is 2^32 bytes
#[derive(PartialEq, Debug)]
pub struct VarLen32(pub Vec<u8>);
impl Ipv8Payload for VarLen32 {}

/// Struct representing a payload section of variable length section of a payload.
/// VarLen16 means the max length of the variable length section is 2^64 bytes
#[derive(PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct VarLen64(pub Vec<u8>);
impl Ipv8Payload for VarLen64 {}

impl<'de> Deserialize<'de> for VarLen16 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VarLen16Visitor;
        impl<'de> Visitor<'de> for VarLen16Visitor {
            type Value = VarLen16;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("VarLen16")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut res: Vec<u8> = vec![];

                // first read the length from the sequence
                let length: u16 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                // now read that many bytes from the sequence
                for _i in 0..length {
                    res.push(
                        seq.next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?,
                    );
                }

                Ok(VarLen16(res))
            }
        }

        //deserialize it as a tuple of maximum length (2^16)
        Ok(deserializer.deserialize_tuple(1 << 16, VarLen16Visitor)?)
    }
}

impl Serialize for VarLen16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let length = self.0.len();
        if length > 0xffff {
            return Err(Error::custom(
                "Data too large to fit in a VarLen16. Must be less than 65536 bytes.",
            ));
        }
        // 2 bytes for the length of the length prefix, as this is a varlen*16*
        // TODO: possible rewrite to serialize_tuple https://docs.rs/serde/1.0.70/serde/ser/trait.SerializeTuple.html
        let mut state = serializer.serialize_struct("", self.0.len() + 2)?;
        state.serialize_field("len", &(length as u16))?;
        for i in &self.0 {
            state.serialize_field("val", &i)?;
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for VarLen32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VarLen32Visitor;
        impl<'de> Visitor<'de> for VarLen32Visitor {
            type Value = VarLen32;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("VarLen16")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut res: Vec<u8> = vec![];

                // first read the length from the sequence
                let length: u32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                // now read that many bytes from the sequence
                for _i in 0..length {
                    res.push(
                        seq.next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?,
                    );
                }

                Ok(VarLen32(res))
            }
        }

        Ok(deserializer.deserialize_tuple(1 << 32, VarLen32Visitor)?)
    }
}

impl Serialize for VarLen32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let length = self.0.len();
        if length > 0xffff_ffff {
            return Err(Error::custom(
                "Data too large to fit in a VarLen32. Must be less than 4294967295 bytes.",
            ));
        }
        // 2 bytes for the length of the length prefix, as this is a varlen*32*
        // TODO: possible rewrite to serialize_tuple https://docs.rs/serde/1.0.70/serde/ser/trait.SerializeTuple.html
        let mut state = serializer.serialize_struct("", self.0.len() + 4)?;
        state.serialize_field("len", &(length as u32))?;
        for i in &self.0 {
            state.serialize_field("val", &i)?;
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialization::Packet;

    #[test]
    fn test_serialize_varlen16() {
        let i = VarLen16(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();
        assert_eq!(
            packet,
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 10, 1, 2,
                3, 4, 5, 6, 7, 8, 9, 10
            ])
        )
    }

    #[test]
    fn test_deserialize_varlen16() {
        let i = VarLen16(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();
        let out: VarLen16 = packet
            .start_deserialize()
            .skip_header()
            .unwrap()
            .next_payload()
            .unwrap();
        assert_eq!(i, out)
    }

    #[test]
    fn test_serialize_varlen32() {
        let i = VarLen32(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();
        assert_eq!(
            packet,
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 10,
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10
            ])
        );
    }

    #[test]
    fn test_deserialize_varlen32() {
        let i = VarLen32(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
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
        )
    }

    #[test]
    fn test_varlen32_large() {
        let mut tmp: Vec<u8> = vec![];
        for i in 0..(1u32 << 17) {
            tmp.push((i % 255) as u8);
        }
        let i = VarLen32(tmp);
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
        )
    }

    #[test]
    fn test_varlen16_too_large() {
        let tmp: Vec<u8> = vec![0; (1u32 << 17) as usize];
        let i = VarLen16(tmp);
        match Packet::new(create_test_header!()).unwrap().add(&i) {
            Ok(_) => assert!(
                false,
                "this should throw an error as 2^17 bytes is too large for a varlen16"
            ),
            Err(_) => assert!(true),
        };
    }

    // fucking ci cant run this
    #[test]
    #[ignore]
    fn test_varlen32_too_large() {
        let tmp: Vec<u8> = vec![0; (1u64 << 32 + 1) as usize];
        let i = VarLen32(tmp);
        match Packet::new(create_test_header!()).unwrap().add(&i) {
            Ok(_) => assert!(
                false,
                "this should throw an error as 2^33 bytes is too large for a varlen32"
            ),
            Err(_) => assert!(true),
        };
    }

    #[test]
    fn test_serialize_varlen16_zero() {
        let i = VarLen16(vec![]);
        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();
        assert_eq!(
            packet,
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0
            ])
        );
    }

}
