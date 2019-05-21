use serde;
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};
use serde::ser::{Error, Serialize, Serializer, SerializeStruct};
use std::fmt;
use crate::networking::payloads::payload::Ipv8Payload;

/// Struct representing a payload section of variable length section of a payload.
/// VarLen16 means the max length of the variable length section is 2^16 bytes
#[derive(PartialEq,Debug)]
struct VarLen16(
  pub Vec<u8>
);
impl Ipv8Payload for VarLen16{}

/// Struct representing a payload section of variable length section of a payload.
/// VarLen16 means the max length of the variable length section is 2^32 bytes
#[derive(PartialEq,Debug)]
struct VarLen32(
  pub Vec<u8>
);
impl Ipv8Payload for VarLen32{}

/// Struct representing a payload section of variable length section of a payload.
/// VarLen16 means the max length of the variable length section is 2^64 bytes
#[derive(PartialEq, Debug, serde::Serialize, serde::Deserialize)]
struct VarLen64(
  pub Vec<u8>
);
impl Ipv8Payload for VarLen64{}

impl<'de> Deserialize<'de> for VarLen16 {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D:Deserializer<'de>
  {
    struct VarLen16Visitor;
    impl<'de> Visitor<'de> for VarLen16Visitor{
      type Value = VarLen16;
      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("VarLen16")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
      {
        let mut res:Vec<u8> = vec![];

        // first read the length from the sequence
        let length:u16 = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

        // now read that many bytes from the sequence
        for _i in 0..length{
          res.push(seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?);
        }

        return Ok(VarLen16(res));
      }
    }

    //deserialize it as a tuple of maximum length (2^16)
    return Ok(deserializer.deserialize_tuple(1<<16,VarLen16Visitor)?)
  }
}

impl Serialize for VarLen16 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
  {
    let length = self.0.len();
    if length > 0xffff{
      return Err(Error::custom("Data too large to fit in a VarLen16. Must be less than 65536 bytes."));
    }
    // 2 bytes for the length of the length prefix, as this is a varlen*16*
    // TODO: possible rewrite to serialize_tuple https://docs.rs/serde/1.0.70/serde/ser/trait.SerializeTuple.html
    let mut state = serializer.serialize_struct("",self.0.len() + 2)?;
    state.serialize_field("len",&(length as u16))?;
    for i in &self.0{
      state.serialize_field("val",&i)?;
    }
    state.end()
  }
}

impl<'de> Deserialize<'de> for VarLen32 {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D:Deserializer<'de>
  {

    struct VarLen32Visitor;
    impl<'de> Visitor<'de> for VarLen32Visitor{
      type Value = VarLen32;
      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("VarLen16")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
      {
        let mut res:Vec<u8> = vec![];

        // first read the length from the sequence
        let length:u32 = seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

        // now read that many bytes from the sequence
        for _i in 0..length{
          res.push(seq.next_element()?.ok_or_else(|| serde::de::Error::invalid_length(1, &self))?);
        }

        return Ok(VarLen32(res));
      }
    }

    return Ok(deserializer.deserialize_tuple(1<<32,VarLen32Visitor)?)
  }
}

impl Serialize for VarLen32 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
  {
    let length = self.0.len();
    if length > 0xffffffff{
      return Err(Error::custom("Data too large to fit in a VarLen32. Must be less than 4294967295 bytes."));
    }
    // 2 bytes for the length of the length prefix, as this is a varlen*32*
    // TODO: possible rewrite to serialize_tuple https://docs.rs/serde/1.0.70/serde/ser/trait.SerializeTuple.html
    let mut state = serializer.serialize_struct("",self.0.len() + 4)?;
    state.serialize_field("len",&(length as u32))?;
    for i in &self.0{
      state.serialize_field("val",&i)?;
    }
    state.end()
  }
}

#[cfg(test)]
mod tests {
  use bincode;

  use super::*;
  use super::super::deserialize;
  use super::super::serialize;

  #[test]
  fn test_serialize_varlen16(){
    let i = VarLen16(vec![1,2,3,4,5,6,7,8,9,10]);
    let ser_tmp = serialize(&i).unwrap();
    assert_eq!(ser_tmp,vec![0,10,1,2,3,4,5,6,7,8,9,10])
  }

  #[test]
  fn test_deserialize_varlen16(){
    let i = VarLen16(vec![1,2,3,4,5,6,7,8,9,10]);
    let ser_tmp = serialize(&i).unwrap();
    assert_eq!(i,deserialize(&ser_tmp).unwrap())
  }

  #[test]
  fn test_serialize_varlen32(){
    let i = VarLen32(vec![1,2,3,4,5,6,7,8,9,10]);
    let ser_tmp = serialize(&i).unwrap();
    assert_eq!(ser_tmp,vec![0,0,0,10,1,2,3,4,5,6,7,8,9,10])
  }

  #[test]
  fn test_deserialize_varlen32(){
    let i = VarLen32(vec![1,2,3,4,5,6,7,8,9,10]);
    let ser_tmp = serialize(&i).unwrap();
    assert_eq!(i,deserialize(&ser_tmp).unwrap())
  }

  #[test]
  fn test_varlen32_large(){
    let mut tmp:Vec<u8> = vec![];
    for i in 0..(1u32<<17){
      tmp.push((i % 255) as u8);
    }
    let i = VarLen32(tmp);
    let ser_tmp = serialize(&i).unwrap();
    assert_eq!(i,deserialize(&ser_tmp).unwrap())
  }

  #[test]
  fn test_varlen16_too_large(){
    let tmp:Vec<u8> = vec![0; (1u32 << 17) as usize];
    let i = VarLen16(tmp);
    match serialize(&i){
      Ok(_) => assert!(false, "this should throw an error as 2^17 bytes is too large for a varlen16"),
      Err(_) => assert!(true)
    };
  }

  // fucking ci cant run this
  #[test]
  #[ignore]
  fn test_varlen32_too_large(){
    let tmp:Vec<u8> = vec![0; (1u64 << 32 + 1) as usize];
    let i = VarLen32(tmp);
    match serialize(&i){
      Ok(_) => assert!(false, "this should throw an error as 2^33 bytes is too large for a varlen32"),
      Err(_) => assert!(true)
    };
  }

  #[test]
  fn test_serialize_varlen16_zero(){
    let i = VarLen16(vec![]);
    let ser_tmp = serialize(&i).unwrap();
    assert_eq!(ser_tmp,vec![0,0])
  }

}
