
use super::bits::Bits;

use std::mem;
use std::vec::Vec;

//macro which does a match for an Option, but returns from the outer function instead of this match.
macro_rules! unwrap_or_return_value {
  ( $e:expr , $v:ident) => {
    match $e {
      Some(x) => x,
      None => return $v,
    }
  };
}

/// Basicaly a union of all possible deserializable datatypes. Return type of the deserialization process.
/// roughly copied from https://docs.python.org/2/library/struct.html#format-characters
/// as py-ipv8 does this too (https://github.com/Tribler/py-ipv8/blob/57c1aa73eee8a3b7ee6ad48482fc2e0d5849415e/ipv8/messaging/serialization.py#L185).
/// Allways packs big-endian.
#[derive(PartialEq, Debug)]
pub enum PacketDataIdentifier {
  CHAR(char),
  BOOL(bool),
  I8(i8),
  U8(u8),
  I16(i16),
  U16(u16),
  I32(i32),
  U32(u32),
  I64(i64),
  U64(u64),
  F32(f32),
  F64(f64),
  STRING(String),
  /// like string but returns byte array
  RAW(Vec<u8>),
  //returns a bits struct (WIP: should have 8 boolean fields corresponding to the bits set).
  BITS(Bits),
}

/// with a vector of these, a format can be specified with which the packet can be decoded.
#[derive(PartialEq, Debug)]
pub enum PacketFormatIdentifier {
  PAD,
  CHAR,
  BOOL,
  I8,
  U8,
  I16,
  U16,
  I32,
  U32,
  I64,
  U64,
  F32,
  F64,
  /// not yet implemented as py-ipv8 doesnt
  STRING,
  /// like string but length is known
  STRINGLEN(u32),
  /// like string but returns byte array
  /// not yet implemented as py-ipv8 doesnt
  RAW,
  /// like raw but length is known
  RAWLEN(u32),
  //returns a bits struct (WIP: should have 8 boolean fields corresponding to the bits set).
  BITS,

  /// combination of the above
  PAYLOAD(Vec<Vec<PacketFormatIdentifier>>),
}

pub type PacketFormat = Vec<Vec<PacketFormatIdentifier>>;

#[derive(Debug)]
pub struct Packet {
  data: Vec<u8>,
}

impl Packet {

  /// create a packet based on a byte array
  pub fn from(data: &[u8]) -> Self {
    Packet {
      data: data.to_vec(),
    }
  }

  /// unpack a packet based on it's format into rust data types.
  pub fn unpack(&self, format: PacketFormat) -> Option<Vec<Vec<PacketDataIdentifier>>> {
    let mut finalres: Vec<Vec<PacketDataIdentifier>> = Vec::new();
    let mut data = self.data.iter();


    for line in &format {
      // inner vector of finalres
      let mut res: Vec<PacketDataIdentifier> = Vec::new();

      for datatype in line {
        if *datatype != PacketFormatIdentifier::PAD {
          res.push(
            // return None if any of the values fails
            unwrap_or_return_value!(
              // match returns some if parsable, else None.
              // there's a match for every possible kind of data in PacketDataIdentifier.
              match *datatype {
                PacketFormatIdentifier::CHAR => Some(PacketDataIdentifier::CHAR(
                  *unwrap_or_return_value!(data.next(), None) as char
                )),
                PacketFormatIdentifier::BOOL => Some(PacketDataIdentifier::BOOL(
                  *unwrap_or_return_value!(data.next(), None) > 0
                )),
                PacketFormatIdentifier::I8 => Some(PacketDataIdentifier::I8(u8::from(
                  *unwrap_or_return_value!(data.next(), None)
                )
                  as i8)),
                PacketFormatIdentifier::U8 => Some(PacketDataIdentifier::U8(u8::from(
                  *unwrap_or_return_value!(data.next(), None)
                ))),
                PacketFormatIdentifier::I16 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u16;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u16;
                  Some(PacketDataIdentifier::I16(
                    u16::from(part0 << 8 | part1) as i16
                  ))
                }
                PacketFormatIdentifier::U16 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u16;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u16;
                  Some(PacketDataIdentifier::U16(u16::from(part0 << 8 | part1)))
                }
                PacketFormatIdentifier::I32 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part2 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part3 = *unwrap_or_return_value!(data.next(), None) as u32;
                  Some(PacketDataIdentifier::I32(u32::from(
                    part0 << 24 | part1 << 16 | part2 << 8 | part3,
                  ) as i32))
                }
                PacketFormatIdentifier::U32 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part2 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part3 = *unwrap_or_return_value!(data.next(), None) as u32;
                  Some(PacketDataIdentifier::U32(u32::from(
                    part0 << 24 | part1 << 16 | part2 << 8 | part3,
                  )))
                }
                PacketFormatIdentifier::I64 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part2 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part3 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part4 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part5 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part6 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part7 = *unwrap_or_return_value!(data.next(), None) as u64;
                  Some(PacketDataIdentifier::I64(u64::from(
                    part0 << 56
                      | part1 << 48
                      | part2 << 40
                      | part3 << 32
                      | part4 << 24
                      | part5 << 16
                      | part6 << 8
                      | part7,
                  ) as i64))
                }
                PacketFormatIdentifier::U64 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part2 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part3 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part4 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part5 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part6 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part7 = *unwrap_or_return_value!(data.next(), None) as u64;
                  Some(PacketDataIdentifier::U64(u64::from(
                    part0 << 56
                      | part1 << 48
                      | part2 << 40
                      | part3 << 32
                      | part4 << 24
                      | part5 << 16
                      | part6 << 8
                      | part7,
                  )))
                }
                PacketFormatIdentifier::F32 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part2 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let part3 = *unwrap_or_return_value!(data.next(), None) as u32;
                  let bits = u32::from(part0 << 24 | part1 << 16 | part2 << 8 | part3);
                  let res: f32 = unsafe { mem::transmute(bits) };
                  Some(PacketDataIdentifier::F32(res))
                }
                PacketFormatIdentifier::F64 => {
                  let part0 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part1 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part2 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part3 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part4 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part5 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part6 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let part7 = *unwrap_or_return_value!(data.next(), None) as u64;
                  let bits = u64::from(
                    part0 << 56
                      | part1 << 48
                      | part2 << 40
                      | part3 << 32
                      | part4 << 24
                      | part5 << 16
                      | part6 << 8
                      | part7,
                  );
                  let res: f64 = unsafe { mem::transmute(bits) };
                  Some(PacketDataIdentifier::F64(res))
                }
                PacketFormatIdentifier::STRINGLEN(len) => {
                  let mut resultstring = String::new();
                  for _ in 0..len {
                    resultstring.push(*unwrap_or_return_value!(data.next(), None) as char);
                  }
                  Some(PacketDataIdentifier::STRING(resultstring))
                }
                PacketFormatIdentifier::RAWLEN(len) => {
                  let mut resultstring: Vec<u8> = Vec::new();
                  for _ in 0..len {
                    resultstring.push(*unwrap_or_return_value!(data.next(), None));
                  }
                  Some(PacketDataIdentifier::RAW(resultstring))
                }
                PacketFormatIdentifier::RAWLEN(len) => Some(PacketDataIdentifier::BITS(
                  Bits::from(*unwrap_or_return_value!(data.next(), None))
                )),
                _ => panic!("Not implemented!"),
              },
              None
            ),
          );
        }
      }
      finalres.push(res);
    }

    //return None if the iterator isn't empty
    if match data.next() {
      Some(_) => true,
      None => false,
    } {
      return None;
    }

    return Some(finalres);
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unpack_identity() {
    let packet = Packet::from(&[0xff, 0xff, 0xff]);

    assert_eq!(
      packet.unpack(vec![vec![
        PacketFormatIdentifier::U8,
        PacketFormatIdentifier::U8,
        PacketFormatIdentifier::U8,
      ]]),
      Some(vec![vec![
        PacketDataIdentifier::U8(0xff),
        PacketDataIdentifier::U8(0xff),
        PacketDataIdentifier::U8(0xff),
      ]])
    );
  }

  #[test]
  fn test_unpack_different_size_16() {
    let packet = Packet::from(&[0xff, 0xff, 0xff]);

    assert_eq!(
      packet.unpack(vec![vec![
        PacketFormatIdentifier::U16,
        PacketFormatIdentifier::U8,
      ]]),
      Some(vec![vec![
        PacketDataIdentifier::U16(0xffff),
        PacketDataIdentifier::U8(0xff),
      ]])
    );
  }

  #[test]
  fn test_unpack_different_size_32() {
    let packet = Packet::from(&[0xff, 0xff, 0xff, 0xff, 0xff]);

    assert_eq!(
      packet.unpack(vec![vec![
        PacketFormatIdentifier::U32,
        PacketFormatIdentifier::U8,
      ]]),
      Some(vec![vec![
        PacketDataIdentifier::U32(0xffffffff),
        PacketDataIdentifier::U8(0xff),
      ]])
    );
  }

  #[test]
  fn test_unpack_different_size_64() {
    let packet = Packet::from(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);

    assert_eq!(
      packet.unpack(vec![vec![
        PacketFormatIdentifier::U64,
        PacketFormatIdentifier::U8,
      ]]),
      Some(vec![vec![
        PacketDataIdentifier::U64(0xffffffffffffffff),
        PacketDataIdentifier::U8(0xff),
      ]])
    );
  }

  #[test]
  fn test_unpack_too_large() {
    let packet = Packet::from(&[0xff]);

    assert_eq!(
      packet.unpack(vec![vec![
        PacketFormatIdentifier::U8,
        PacketFormatIdentifier::U8,
      ]]),
      None
    );
  }

  #[test]
  fn test_unpack_too_small() {
    let packet = Packet::from(&[0xff, 0xff, 0xff]);

    assert_eq!(
      packet.unpack(vec![vec![
        PacketFormatIdentifier::U8,
        PacketFormatIdentifier::U8,
      ]]),
      None
    );
  }

  #[test]
  fn test_unpack_string() {
    let packet = Packet::from(&[52, 50]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::STRINGLEN(2),]]),
      Some(vec![
        vec![PacketDataIdentifier::STRING(String::from("42")),]
      ])
    );
  }

  #[test]
  fn test_unpack_raw() {
    let packet = Packet::from(&[52, 50]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::RAWLEN(2),]]),
      Some(vec![vec![PacketDataIdentifier::RAW([52, 50].to_vec()),]])
    );
  }

  #[test]
  fn test_unpack_bool_true() {
    let packet = Packet::from(&[1]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::BOOL,]]),
      Some(vec![vec![PacketDataIdentifier::BOOL(true),]])
    );
  }

  #[test]
  fn test_unpack_bool_false() {
    let packet = Packet::from(&[0]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::BOOL,]]),
      Some(vec![vec![PacketDataIdentifier::BOOL(false),]])
    );
  }

  #[test]
  fn test_unpack_bool_large() {
    let packet = Packet::from(&[42]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::BOOL,]]),
      Some(vec![vec![PacketDataIdentifier::BOOL(true),]])
    );
  }

  #[test]
  fn test_big_endian() {
    let packet = Packet::from(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::U64,]]),
      Some(vec![vec![PacketDataIdentifier::U64(0x0123456789abcdef),]])
    );
  }

  #[test]
  fn test_big_signed() {
    let packet = Packet::from(&[(-1i8) as u8]);

    assert_eq!(
      packet.unpack(vec![vec![PacketFormatIdentifier::I8,]]),
      Some(vec![vec![PacketDataIdentifier::I8(-1),]])
    );
  }
}

