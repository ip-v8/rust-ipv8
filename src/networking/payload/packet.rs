use super::bits::Bits;
use std::iter;
use std::mem;
use std::slice;

//macro which does a match for an Option, but returns from the outer function instead of this match.
macro_rules! unwrap_or_return_value {
  ( $e:expr , $v:ident) => {
    match $e {
      Some(x) => x,
      None => return $v,
    }
  };
}

#[derive(Debug)]
pub struct Packet {
  data: Vec<u8>,
}

pub struct PacketIterator<'a> {
  i: iter::Peekable<slice::Iter<'a, u8>>,
}

impl<'a> PacketIterator<'a> {
  pub fn next_char(&mut self) -> Option<char> {
    match self.i.next() {
      Some(&i) => Some(i as char),
      None => None,
    }
  }

  pub fn next_bool(&mut self) -> Option<bool> {
    match self.i.next() {
      Some(&i) => Some(i > 0),
      None => None,
    }
  }

  pub fn next_i8(&mut self) -> Option<i8> {
    match self.i.next() {
      Some(&i) => Some(i as i8),
      None => None,
    }
  }

  pub fn next_u8(&mut self) -> Option<u8> {
    match self.i.next() {
      Some(&i) => Some(i as u8),
      None => None,
    }
  }

  pub fn next_i16(&mut self) -> Option<i16> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u16;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u16;
    Some((part0 << 8 | part1) as i16)
  }

  pub fn next_u16(&mut self) -> Option<u16> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u16;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u16;
    Some((part0 << 8 | part1) as u16)
  }

  pub fn next_i32(&mut self) -> Option<i32> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part2 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part3 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    Some((part0 << 24 | part1 << 16 | part2 << 8 | part3) as i32)
  }

  pub fn next_u32(&mut self) -> Option<u32> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part2 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part3 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    Some((part0 << 24 | part1 << 16 | part2 << 8 | part3) as u32)
  }

  pub fn next_i64(&mut self) -> Option<i64> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part2 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part3 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part4 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part5 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part6 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part7 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    Some(
      (part0 << 56
        | part1 << 48
        | part2 << 40
        | part3 << 32
        | part4 << 24
        | part5 << 16
        | part6 << 8
        | part7) as i64,
    )
  }

  pub fn next_u64(&mut self) -> Option<u64> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part2 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part3 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part4 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part5 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part6 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part7 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    Some(
      (part0 << 56
        | part1 << 48
        | part2 << 40
        | part3 << 32
        | part4 << 24
        | part5 << 16
        | part6 << 8
        | part7) as u64,
    )
  }

  pub fn next_f32(&mut self) -> Option<f32> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part2 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let part3 = *unwrap_or_return_value!(self.i.next(), None) as u32;
    let bits = (part0 << 24 | part1 << 16 | part2 << 8 | part3) as u32;
    let res: f32 = unsafe { mem::transmute(bits) };
    Some(res)
  }

  pub fn next_f64(&mut self) -> Option<f64> {
    let part0 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part1 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part2 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part3 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part4 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part5 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part6 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let part7 = *unwrap_or_return_value!(self.i.next(), None) as u64;
    let bits = (part0 << 56
      | part1 << 48
      | part2 << 40
      | part3 << 32
      | part4 << 24
      | part5 << 16
      | part6 << 8
      | part7) as u64;
    let res: f64 = unsafe { mem::transmute(bits) };
    Some(res)
  }

  pub fn next_string(&mut self, length: u32) -> Option<String> {
    let mut resultstring = String::new();
    for _ in 0..length {
      resultstring.push(*unwrap_or_return_value!(self.i.next(), None) as char);
    }
    Some(resultstring)
  }

  pub fn next_raw(&mut self, length: u32) -> Option<Vec<u8>> {
    let mut resultstring: Vec<u8> = Vec::new();
    for _ in 0..length {
      resultstring.push(*unwrap_or_return_value!(self.i.next(), None));
    }
    Some(resultstring)
  }

  pub fn next_bits(&mut self, length: u32) -> Option<Bits> {
    let byte = *unwrap_or_return_value!(self.i.next(), None);
    return Some(Bits::from(byte));
  }

  pub fn done(&mut self) -> bool {
    self.i.peek().is_none()
  }
}

impl Packet {
  /// create a packet based on a byte array
  pub fn from(data: &[u8]) -> Self {
    Packet {
      data: data.to_vec(),
    }
  }

  pub fn iter(&self) -> PacketIterator {
    PacketIterator {
      i: self.data.iter().peekable(),
    }
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unpack_identity() {
    let packet = Packet::from(&[0xff, 0xff, 0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_different_size_16() {
    let packet = Packet::from(&[0xff, 0xff, 0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u16().unwrap(), 0xffff);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_different_size_32() {
    let packet = Packet::from(&[0xff, 0xff, 0xff, 0xff, 0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u32().unwrap(), 0xffffffff);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_different_size_64() {
    let packet = Packet::from(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u64().unwrap(), 0xffffffffffffffffu64);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_too_large() {
    let packet = Packet::from(&[0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.next_u8().is_none(), true);
  }

  #[test]
  fn test_unpack_too_small() {
    let packet = Packet::from(&[0xff, 0xff, 0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.done(), false);
  }

  #[test]
  fn test_unpack_string() {
    let packet = Packet::from(&[52, 50]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_string(2).unwrap(), "42");
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_raw() {
    let packet = Packet::from(&[52, 50]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_raw(2).unwrap(), vec![52, 50]);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_bool_true() {
    let packet = Packet::from(&[1]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_bool().unwrap(), true);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_bool_false() {
    let packet = Packet::from(&[0]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_bool().unwrap(), false);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_bool_large() {
    let packet = Packet::from(&[42]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_bool().unwrap(), true);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_big_endian() {
    let packet = Packet::from(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u64().unwrap(), 0x0123456789abcdef);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_signed() {
    let packet = Packet::from(&[(-1i8) as u8]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_i8().unwrap(), -1);
    assert_eq!(packetiter.done(), true);
  }
}
