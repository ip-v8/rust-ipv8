use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::marker::PhantomData;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;
use std::io::Cursor;

/// Datatype representing the raw bytes at the end of an ipv8 payload where the length shouldnt be prefixed.
#[derive(Debug, PartialEq)]
pub struct RawEnd(pub Vec<u8>);


impl Serialize for RawEnd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer{
        let mut state = serializer.serialize_struct("Bits", self.0.len())?;
        for i in &self.0{
          state.serialize_field("value", &i)?;
        }
        state.end()
    }
}


/// used for deserializing IntroductionRequestPayload
struct RawEndVisitor{
  marker: PhantomData<fn() -> RawEnd>
}

struct u8Visitor{
  marker: PhantomData<fn() -> u8>
}

impl<'de> Visitor<'de> for u8Visitor{
  type Value = u8;
  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("RawEnd")
  }
}

impl<'de> Visitor<'de> for RawEndVisitor{
  type Value = u8;
  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("RawEnd")
  }
}


impl<'de> Deserialize<'de> for RawEnd{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where D: Deserializer<'de>,{
    let result:Vec<u8> = Vec::new();


    let mut input = Cursor::new();
    // let visitor = RawEndVisitor{marker:PhantomData};
    // let deserializer1 = &deserializer;

    // while true{
    //   let value = (&deserializer).deserialize_u8(u8Visitor {marker:PhantomData});
    //   match value{
    //     Ok(i) => result.push(i),
    //     Err(i) => break,
    //   }
    // }
    // deserializer.


    Ok(RawEnd(result))
  }
}
