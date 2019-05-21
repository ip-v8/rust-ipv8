use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde;
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};
use std::fmt;

/// Datatype representing the raw bytes at the end of an ipv8 payload where the length shouldn't be prefixed.
#[derive(Debug, PartialEq)]
pub struct RawEnd (
  pub Vec<u8>
);

impl Serialize for RawEnd {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: Serializer{
    let mut state = serializer.serialize_struct("RawEnd", self.0.len())?;
    for i in &self.0{
      state.serialize_field("value", &i)?;
    }
    state.end()
  }
}

impl<'de> Deserialize<'de> for RawEnd {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D:Deserializer<'de>
  {
    struct RawEndVisitor;
    impl<'de> Visitor<'de> for RawEndVisitor{
      type Value = RawEnd;
      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("RawEnd")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
      {
        let mut res:Vec<u8> = vec![];

        loop {
          match seq.next_element(){
            Ok(item) => match item{
                Some(value) => res.push(value),
                None => break
            },
            Err(_err) => break
          }
        }
        return Ok(RawEnd(res));
      }
    }
    //TODO: something like infinity?
    return Ok(deserializer.deserialize_tuple(std::usize::MAX,RawEndVisitor)?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde::{Serialize,Deserialize};
  use crate::networking::payloads::payload::Ipv8Payload;
  use crate::networking::serialization::Packet;

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct TestPayload1 {
    test:RawEnd,
  }

  impl Ipv8Payload for TestPayload1 {
    // doesnt have anything but needed for the default implementation (as of right now)
  }

  # [test]
  fn test_serialize_rawend(){
    let a = TestPayload1{test:RawEnd(vec![42,43])};

    let ser_tmp = Packet::serialize(&a).unwrap();

    assert_eq!(Packet(vec![42,43]),ser_tmp);
  }

  # [test]
  fn test_deserialize_rawend(){
    let a = TestPayload1{test:RawEnd(vec![42,43,])};

    let mut ser_tmp = Packet::serialize(&a).unwrap();
    assert_eq!(a,ser_tmp.deserialize().unwrap());
  }
}
