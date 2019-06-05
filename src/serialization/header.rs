use std::fmt;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};
use serde::ser::SerializeTuple;

#[derive(PartialEq, Debug)]
pub enum HeaderVersion {
  PyIPV8Header,
}

#[derive(PartialEq, Debug)]
pub struct Header {
  pub size: usize, // This is a size that is hardcoded when a header is deserialized.
  pub version: HeaderVersion,
  pub mid_hash: Option<Vec<u8>>,
  pub message_type: Option<u64>,
}

impl Header{
  pub fn py_ipv8_header(mid_hash: Vec<u8>, message_type: u64) -> Self {
    Header{
      size: PY_IPV8_HEADER_SIZE,
      version: HeaderVersion::PyIPV8Header,
      mid_hash: Some(mid_hash),
      message_type: Some(message_type),
    }
  }
}

//------------start header constants------------

/// This is the pattern of the PyIPV8header in individual bytes, this is needed for the Deserializer
/// to turn the raw bytes into the a temporary struct fort then to be turned into the actual Header
/// The more observant among you may have noticed the lack of bytes for the version string
/// this is because we take those of byte by byte manually as this indicates which header we have and
/// thus can not be done generically.
#[derive(Debug, PartialEq, serde::Deserialize)]
struct PyIPV8HeaderPattern(
  // No version, as this is removed without the pattern
  u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, // mid hash (always 20*u8)
  u8, // message type
);

// 2 bytes magic, 20 bytes hash, 1 byte message type
const PY_IPV8_HEADER_SIZE: usize = 23;

//------------end header constants------------

/// makes the BinMemberAuthenticationPayload serializable.
/// This is less than trivial as there is no 1:1 mapping between the serialized data and the payload struct.
/// Some struct fields are combined into one byte to form the serialized data.
impl Serialize for Header {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
    match self.version {
      HeaderVersion::PyIPV8Header => {
        // 23 is the size of the Python IPV8 Header.
        let mut state = serializer.serialize_tuple(PY_IPV8_HEADER_SIZE)?;
        match self.version{
          HeaderVersion::PyIPV8Header => state.serialize_element(&(0002 as u16))?,
        }
        for i in &self.mid_hash.clone() {
          state.serialize_element(&i)?;
        }
        state.serialize_element(&(self.message_type))?;
        state.end()
      }
    }
  }
}

impl<'de> Deserialize<'de> for Header {
  /// deserializes an IntroductionRequestPayload
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
    // first deserialize it to a temporary struct which literally represents the packer

    struct HeaderVisitor;
    impl<'de> Visitor<'de> for HeaderVisitor {
      type Value = Header;
      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Header")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where A: SeqAccess<'de>
      {
        let mut version = None;
        let mut version_bytes: Vec<u8> = vec![];

        loop { // this block is here to be breaked out of or else return an error
          version_bytes.push(match seq.next_element()? {Some(i) => i, None => return Err(
            serde::de::Error::custom("No valid header type could be determined"))
          });
          version_bytes.push(match seq.next_element()? {Some(i) => i, None => return Err(
            serde::de::Error::custom("No valid header type could be determined"))
          });

          if version_bytes.as_slice() == [0, 2] {
            version = Some(HeaderVersion::PyIPV8Header);
            break;
          }

          // version_bytes.push(match seq.next_element()? {Some(i) => i, None => return Err(
          //   serde::de::Error::custom("No valid header type could be determined"))
          // });
          // version_bytes.push(match seq.next_element()? {Some(i) => i, None => return Err(
          //   serde::de::Error::custom("No valid header type could be determined"))
          // });
          // add checks for larger headers here
          return Err(serde::de::Error::custom("No valid header type could be determined"));
        }

        match version {
          Some(i) => match i {
            HeaderVersion::PyIPV8Header => {
              let mut mid_hash: Vec<u8> = vec![];
              // Where 20 is the length of the mid hash
              for _ in 0..20{
                mid_hash.push(match seq.next_element()? {Some(i) => i, None => return Err(
                  serde::de::Error::custom("No valid header type could be determined"))
                });
              }

              let message_type = match seq.next_element()? {Some(i) => i, None => return Err(
                serde::de::Error::custom("No valid header type could be determined"))
              };

              Ok(Header::py_ipv8_header(
                mid_hash,
                message_type,
              ))
            }
          },
          None => return Err(serde::de::Error::custom("Somehow the header type was valid but the version None"))
        }
      }
    }

    Ok(deserializer.deserialize_tuple(std::usize::MAX, HeaderVisitor)?)
    
  }
}

macro_rules! create_test_header {
  () => {
    crate::serialization::header::Header {
      size: 23,
      version: crate::serialization::header::HeaderVersion::PyIPV8Header,
      mid_hash: Some(vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]),
      message_type: Some(43)
    };
  };
}

#[cfg(test)]
mod tests {
  use bincode;

  use super::*;

  #[test]
  fn integration_test_creation() {
    let h = Header::py_ipv8_header(
      vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
      42
    );

    assert_eq!(h, bincode::config().big_endian().deserialize(
      &bincode::config().big_endian().serialize(&h).unwrap()
    ).unwrap());
  }
}
