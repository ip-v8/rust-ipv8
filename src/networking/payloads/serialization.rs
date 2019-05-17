use super::payload::Ipv8Payload;
use std::io::Cursor;
use bincode;

pub fn deserialize<T>(buffer: &[u8]) -> <T> {
  let cur = Cursor::new(buffer);
  let msg = bincode::deserialize_from(cur);


}


