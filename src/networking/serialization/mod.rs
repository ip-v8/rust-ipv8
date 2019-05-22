pub mod bits;
pub mod rawend;
pub mod varlen;
pub mod nestedpayload;

use bincode;
use crate::networking::payloads::Ipv8Payload;
use bincode::ErrorKind;
use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize, PartialEq)]
pub struct Packet(
  pub Vec<u8>,
);

#[derive(Debug, PartialEq)]
pub struct PacketIterator{
  pub pntr: Packet,
  pub index: usize,
}

/// iterates over a packet to extract it's possibly multiple payloads
impl PacketIterator{
  /// Deserializes a stream of bytes into an ipv8 payload. Which payload is inferred by the type of T which is generic.
  /// T has to be deserializable and implement the Ipv8Payload trait.
  pub fn next<T>(&mut self) -> Result<T, Box<ErrorKind>>
    where for<'de> T: Deserialize<'de> + Ipv8Payload + Serialize
  {
    let res: T = bincode::config().big_endian().deserialize(&self.pntr.0[self.index ..])?;

    // the old solution was: self.index += size_of::<T>();
    // this doesnt work as it is not uncommon to return less bytes than was actually in the bytecode (lengths etc)
    // the code below works but is inefficient. TODO: create a more efficient way to do this.
    // tried this:
    /*
      let mut value = &self.pntr.0[self.index ..];
      let oldsize = value.len();
      let res: T = bincode::config().big_endian().deserialize_from(&mut value)?;
      self.index += (oldsize - value.to_owned().len());
    */
    // apparently it is less efficient than recalculating the size as below.
    // on the bench_deserialize_multiple benchmark in the tests section below
    // it got 17,584,554 ns per iteration (where each iteration is 100000 serialize/deserializations
    // while the recalculation takes 11,965,294ns
    self.index += bincode::config().big_endian().serialized_size(&res)? as usize;

    Ok(res)
  }

  fn len(&self) -> usize {
    self.pntr.0.len()
  }
}

impl Packet{
  pub fn new() -> Self{
    Self(vec![])
  }

  /// Deserializes a stream of bytes into an ipv8 payload. Which payload is inferred by the type of T which is generic.
  /// T has to be deserializable and implement the Ipv8Payload trait.
  /// Only deserializes one (and the first) payload in a packet. Use the deserialize_multiple function with the PacketIterator for more payloads.
  pub fn deserialize<T>(&mut self) -> Result<T, Box<ErrorKind>>
    where for<'de> T: Deserialize<'de> + Ipv8Payload
  {
    let res: T = bincode::config().big_endian().deserialize(&self.0[..])?;
    Ok(res)
  }

  /// Used for deeserializing multiple payloads.
  pub fn deserialize_multiple(self) -> PacketIterator
  {
    PacketIterator{
      pntr : self,
      index : 0,
    }
  }

  /// simple wrapper function to serialize to bincode. TODO: how will we handle serialization to other standards like json easily?
  pub fn serialize<T>(obj: &T) -> Result<Self, Box<ErrorKind>>
    where T: Ipv8Payload + Serialize {
    Ok(Self(
      bincode::config().big_endian().serialize(&obj)?
    ))
  }

  pub fn add<T>(&mut self, obj: &T) -> Result<()
    , Box<ErrorKind>>
    where T: Ipv8Payload + Serialize {

    self.0.extend(bincode::config().big_endian().serialize(&obj)?);
    Ok(())
  }

  fn len(&self) -> usize {
    self.0.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde::{Serialize,Deserialize};

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct TestPayload1 {
    test:u16,
  }

  impl Ipv8Payload for TestPayload1 {
    // doesnt have anything but needed for the default implementation (as of right now)
  }

  #[derive(Debug, PartialEq, Serialize, Deserialize)]
  struct TestPayload2 {
    test:u32,
  }

  impl Ipv8Payload for TestPayload2 {
    // doesnt have anything but needed for the default implementation (as of right now)
  }

// only works with feature(test) and with `extern crate test; use test::Bencher`
//  #[bench]
//  fn bench_deserialize_multiple(b: &mut Bencher){
//    b.iter(|| {
//      let n = test::black_box(100000);
//      for i in 0..n{
//        let a = TestPayload1{test:42};
//        let b = TestPayload2{test:43};
//        let c = TestPayload1{test:44};
//
//        let mut ser_tmp = Packet::serialize(&a).unwrap();
//        ser_tmp.add(&b).unwrap();
//        ser_tmp.add(&c).unwrap();
//
//        let mut deser_iterator = ser_tmp.deserialize_multiple();
//        assert_eq!(a,deser_iterator.next().unwrap());
//        assert_eq!(b,deser_iterator.next().unwrap());
//        assert_eq!(c,deser_iterator.next().unwrap());
//      }
//    });
//  }

  # [test]
  fn test_serialize_multiple(){
    let a = TestPayload1{test:42};
    let b = TestPayload2{test:43};
    let c = TestPayload1{test:44};

    let mut ser_tmp = Packet::serialize(&a).unwrap();
    ser_tmp.add(&b).unwrap();
    ser_tmp.add(&c).unwrap();

    assert_eq!(Packet(vec![0, 42, 0, 0, 0, 43, 0, 44]),ser_tmp);
  }

  # [test]
  fn test_deserialize_multiple(){
    let a = TestPayload1{test:42};
    let b = TestPayload2{test:43};
    let c = TestPayload1{test:44};

    let mut ser_tmp = Packet::serialize(&a).unwrap();
    ser_tmp.add(&b).unwrap();
    ser_tmp.add(&c).unwrap();

    let mut deser_iterator = ser_tmp.deserialize_multiple();
    assert_eq!(a,deser_iterator.next().unwrap());
    assert_eq!(b,deser_iterator.next().unwrap());
    assert_eq!(c,deser_iterator.next().unwrap());
  }

  # [test]
  fn test_deserialize_multiple_more(){
    let a = TestPayload1{test:42};
    let b = TestPayload2{test:43};
    let c = TestPayload1{test:44};

    let mut ser_tmp = Packet::serialize(&a).unwrap();
    ser_tmp.add(&b).unwrap();
    ser_tmp.add(&c).unwrap();


    let mut deser_iterator = ser_tmp.deserialize_multiple();
    assert_eq!(a,deser_iterator.next().unwrap());
    assert_eq!(b,deser_iterator.next().unwrap());
    assert_eq!(c,deser_iterator.next().unwrap());

    let last:Result<TestPayload1,Box<ErrorKind>> = deser_iterator.next();
    match last {
      Ok(_) => assert!(false, "this should throw an error as there is no next"),
      Err(_) => assert!(true)
    };
  }
}
