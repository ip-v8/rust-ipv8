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

#[derive(Default, Debug, PartialEq)]
pub struct Packet {
  pub data: Vec<u8>,
}

impl Packet {
  pub fn new() -> Self {
    Packet { data: vec![] }
  }


  pub fn add_char(&mut self, data: char) -> &mut Self {
    self.data.push(data as u8);
    self
  }

  pub fn add_bool(&mut self, data: bool) -> &mut Self {
    self.data.push(if data { 1 } else { 0 });
    self
  }

  pub fn add_u8(&mut self, data: u8) -> &mut Self {
    self.data.push(data as u8);
    self
  }

  pub fn add_i8(&mut self, data: i8) -> &mut Self {
    self.data.push(data as u8);
    self
  }

  pub fn add_u16(&mut self, data: u16) -> &mut Self {
    let part0 = ((data as u16) & 0xff) as u8;
    let part1 = ((data as u16 >> 8) & 0xff) as u8;
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_i16(&mut self, data: i16) -> &mut Self {

    let part0 = ((data as u16) & 0xff) as u8;
    let part1 = ((data as u16 >> 8) & 0xff) as u8;
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_u32(&mut self, data: u32) -> &mut Self {
    let part0 = ((data as u32) & 0xff) as u8;
    let part1 = ((data as u32 >> 8) & 0xff) as u8;
    let part2 = ((data as u32 >> 16) & 0xff) as u8;
    let part3 = ((data as u32 >> 24) & 0xff) as u8;
    self.data.push(part3 as u8);
    self.data.push(part2 as u8);
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_i32(&mut self, data: i32) -> &mut Self {
    let part0 = ((data as u32) & 0xff) as u8;
    let part1 = ((data as u32 >> 8) & 0xff) as u8;
    let part2 = ((data as u32 >> 16) & 0xff) as u8;
    let part3 = ((data as u32 >> 24) & 0xff) as u8;
    self.data.push(part3 as u8);
    self.data.push(part2 as u8);
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_u64(&mut self, data: u64) -> &mut Self {
    let part0 = ((data as u64) & 0xff) as u8;
    let part1 = ((data as u64 >> 8) & 0xff) as u8;
    let part2 = ((data as u64 >> 16) & 0xff) as u8;
    let part3 = ((data as u64 >> 24) & 0xff) as u8;
    let part4 = ((data as u64 >> 32) & 0xff) as u8;
    let part5 = ((data as u64 >> 40) & 0xff) as u8;
    let part6 = ((data as u64 >> 48) & 0xff) as u8;
    let part7 = ((data as u64 >> 56) & 0xff) as u8;
    self.data.push(part7 as u8);
    self.data.push(part6 as u8);
    self.data.push(part5 as u8);
    self.data.push(part4 as u8);
    self.data.push(part3 as u8);
    self.data.push(part2 as u8);
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_i64(&mut self, data: i64) -> &mut Self {
    let part0 = ((data as u64) & 0xff) as u8;
    let part1 = ((data as u64 >> 8) & 0xff) as u8;
    let part2 = ((data as u64 >> 16) & 0xff) as u8;
    let part3 = ((data as u64 >> 24) & 0xff) as u8;
    let part4 = ((data as u64 >> 32) & 0xff) as u8;
    let part5 = ((data as u64 >> 40) & 0xff) as u8;
    let part6 = ((data as u64 >> 48) & 0xff) as u8;
    let part7 = ((data as u64 >> 56) & 0xff) as u8;
    self.data.push(part7 as u8);
    self.data.push(part6 as u8);
    self.data.push(part5 as u8);
    self.data.push(part4 as u8);
    self.data.push(part3 as u8);
    self.data.push(part2 as u8);
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_f32(&mut self, data: f32) -> &mut Self {
    let res: u32 = unsafe { mem::transmute(data) };
    let part0 = ((res as u32) & 0xff) as u8;
    let part1 = ((res as u32 >> 8) & 0xff) as u8;
    let part2 = ((res as u32 >> 16) & 0xff) as u8;
    let part3 = ((res as u32 >> 24) & 0xff) as u8;
    self.data.push(part3 as u8);
    self.data.push(part2 as u8);
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_f64(&mut self, data: f64) -> &mut Self {
    let res: u64 = unsafe { mem::transmute(data) };
    let part0 = ((res as u64) & 0xff) as u8;
    let part1 = ((res as u64 >> 8) & 0xff) as u8;
    let part2 = ((res as u64 >> 16) & 0xff) as u8;
    let part3 = ((res as u64 >> 24) & 0xff) as u8;
    let part4 = ((res as u64 >> 32) & 0xff) as u8;
    let part5 = ((res as u64 >> 40) & 0xff) as u8;
    let part6 = ((res as u64 >> 48) & 0xff) as u8;
    let part7 = ((res as u64 >> 56) & 0xff) as u8;
    self.data.push(part7 as u8);
    self.data.push(part6 as u8);
    self.data.push(part5 as u8);
    self.data.push(part4 as u8);
    self.data.push(part3 as u8);
    self.data.push(part2 as u8);
    self.data.push(part1 as u8);
    self.data.push(part0 as u8);
    self
  }

  pub fn add_string(&mut self, data: String, length: u32) -> &mut Self {
    for i in 0..length {
      self.data.push(data.as_bytes()[i as usize] as u8);
    }
    self
  }

  pub fn add_raw(&mut self, data: Vec<u8>, length: u32) -> &mut Self {
    for i in 0..length {
      self.data.push(data[i as usize] as u8);
    }
    self
  }

  pub fn add_raw_remaining(&mut self, data: Vec<u8>) -> &mut Self {
    self.data.extend(&data);
    self
  }

  pub fn add_bits(&mut self, data: Bits) -> &mut Self {
    self.data.push(data.to_u8());
    self
  }
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
    let part0 = u16::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u16::from(*unwrap_or_return_value!(self.i.next(), None));
    Some((part0 << 8 | part1) as i16)
  }

  pub fn next_u16(&mut self) -> Option<u16> {
    let part0 = u16::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u16::from(*unwrap_or_return_value!(self.i.next(), None));
    Some((part0 << 8 | part1) as u16)
  }

  pub fn next_i32(&mut self) -> Option<i32> {
    let part0 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    let part2 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    let part3 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    Some((part0 << 24 | part1 << 16 | part2 << 8 | part3) as i32)
  }

  pub fn next_u32(&mut self) -> Option<u32> {
    let part0 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    let part2 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    let part3 = u32::from(*unwrap_or_return_value!(self.i.next(), None));
    Some((part0 << 24 | part1 << 16 | part2 << 8 | part3) as u32)
  }

  pub fn next_i64(&mut self) -> Option<i64> {
    let part0 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part2 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part3 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part4 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part5 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part6 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part7 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
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
    let part0 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part2 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part3 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part4 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part5 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part6 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part7 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
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
    let part0 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part2 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part3 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let bits = (part0 << 24 | part1 << 16 | part2 << 8 | part3) as u32;
    let res: f32 = f32::from_bits(bits);
    Some(res)
  }

  pub fn next_f64(&mut self) -> Option<f64> {
    let part0 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part1 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part2 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part3 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part4 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part5 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part6 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let part7 = u64::from(*unwrap_or_return_value!(self.i.next(), None));
    let bits = (part0 << 56
      | part1 << 48
      | part2 << 40
      | part3 << 32
      | part4 << 24
      | part5 << 16
      | part6 << 8
      | part7) as u64;
    let res: f64 = f64::from_bits(bits);
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

  pub fn next_raw_remaining(&mut self) -> Option<Vec<u8>> {

    let mut resultstring: Vec<u8> = Vec::new();
    while !self.done() {
      resultstring.push(*unwrap_or_return_value!(self.i.next(), None));
    }
    Some(resultstring)
  }


  pub fn next_bits(&mut self) -> Option<Bits> {
    let byte = *unwrap_or_return_value!(self.i.next(), None);
    Some(Bits::from_u8(byte))
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

    assert_eq!(packetiter.next_u32().unwrap(), 0xffff_ffff);
    assert_eq!(packetiter.next_u8().unwrap(), 0xff);
    assert_eq!(packetiter.done(), true);
  }

  #[test]
  fn test_unpack_different_size_64() {
    let packet = Packet::from(&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    let mut packetiter = packet.iter();

    assert_eq!(packetiter.next_u64().unwrap(), 0xffff_ffff_ffff_ffffu64);
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

    assert_eq!(packetiter.next_u64().unwrap(), 0x0123_4567_89ab_cdefu64);
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
