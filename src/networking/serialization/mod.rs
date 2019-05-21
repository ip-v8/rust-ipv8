pub mod bits;
pub mod rawend;
pub mod varlen;

use bincode;
use crate::networking::payloads::payload::Ipv8Payload;
use bincode::ErrorKind;
use serde::de::Deserialize;
use crate::networking::serialization::rawend::RawEnd;
use serde::ser::Serialize;

/// Deserializes a stream of bytes into an ipv8 payload. Which payload is inferred by the type of T which is generic.
/// T has to be deserializable and implement the Ipv8Payload trait.
pub fn deserialize<T>(buffer: &[u8]) -> Result<T, Box<ErrorKind>>
  where for<'de> T: Deserialize<'de> + Ipv8Payload {
  let mut cur = &buffer[..];

  let mut msg: T = bincode::config().big_endian().deserialize_from(&mut cur)?;
  msg.set_rawend(RawEnd(cur.to_owned()));

  Ok(msg)
}

/// simple wrapper function to serialize to bincode. TODO: how will we handle serialization to other standards like json easily?
pub fn serialize<T>(obj: &T) -> Result<Vec<u8>,Box<ErrorKind>>
  where T: Ipv8Payload + Serialize {
  bincode::config().big_endian().serialize(&obj)
}



