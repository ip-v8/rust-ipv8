#![macro_use]
//! Module containing everything related to headers
use std::fmt;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::ser::{Serialize, Serializer};

/// Enum containg all the types of headers
#[derive(PartialEq, Debug)]
pub enum HeaderVersion {
    /// The PyIPv8 header
    PyIPV8Header,
}

/// The struct for headers.
///
/// The `mid_hash` and `message_type` are Options and larger then they need to
/// be for easier expansion later on.
#[derive(PartialEq, Debug)]
pub struct Header {
    /// This is a size that is hardcoded when a header is deserialized.
    pub size: usize,
    /// The version of the packet, 1 is dispersy and 2 is py-ipv8, could be increased to 3 for rust specific enhancements in the future
    pub version: HeaderVersion,
    /// The hash of the master peer of a community, used to identify to which community the packets belongs
    pub mid_hash: Option<Vec<u8>>,
    /// Specifies the type of messsage, can be used by communities to distinguish between packets
    pub message_type: Option<u64>,
}

impl Header {
    /// Helper function for creating a PyIPv8 compliant Header
    pub fn py_ipv8_header(mid_hash: [u8; 20], message_type: u8) -> Self {
        Header {
            size: PY_IPV8_HEADER_SIZE,
            version: HeaderVersion::PyIPV8Header,
            mid_hash: Some(mid_hash.to_vec()),
            message_type: Some(u64::from(message_type)),
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
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8,
    u8, // mid hash (always 20*u8)
    u8, // message type
);

/// 2 bytes magic + 20 bytes hash + 1 byte message type = 23 bytes
const PY_IPV8_HEADER_SIZE: usize = 23;

//------------end header constants------------

/// makes the Header serializable.
impl Serialize for Header {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        //! All types of headers need to be serialized differently
        match self.version {
            HeaderVersion::PyIPV8Header => {
                let mut state = serializer.serialize_tuple(PY_IPV8_HEADER_SIZE)?;
                match self.version {
                    HeaderVersion::PyIPV8Header => state.serialize_element(&(2 as u16))?,
                }

                // Unwrap the hash
                let hash = match &self.mid_hash {
                    Some(m) => m,
                    None => {
                        return Err(serde::ser::Error::custom(
                            "mid_hash was empty and this wasn't expected",
                        ))
                    }
                };

                // Serialize the hash
                for i in hash {
                    state.serialize_element(&i)?;
                }

                // unwrap the message type
                let message_type: u8 = self.message_type.ok_or_else(|| {
                    serde::ser::Error::custom("Message type was empty and this wasn't expected")
                })? as u8;

                // Serialize the message type
                state.serialize_element(&message_type)?;
                state.end()
            }
        }
    }
}

// TODO: Comments
impl<'de> Deserialize<'de> for Header {
    /// deserializes a Header
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // first deserialize it to a temporary struct which literally represents the packer
        #[doc(hidden)]
        struct HeaderVisitor;
        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Header")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let version;
                let mut version_bytes: Vec<u8> = vec![];

                #[allow(clippy::never_loop)]
                // We will make this a proper named block when that feature gets mainlined
                loop {
                    // this block is here to be breaked out of or else return an error

                    // Deserialize the first two bytes into `version_bytes`
                    version_bytes.push(seq.next_element()?.ok_or_else(|| {
                        serde::de::Error::custom("No valid header type could be determined")
                    })?);
                    version_bytes.push(seq.next_element()?.ok_or_else(|| {
                        serde::de::Error::custom("No valid header type could be determined")
                    })?);

                    // Check if they match the header version of PyIPv8: `0002`, if so set it and break out of
                    // the pseudo-loop
                    if version_bytes.as_slice() == [0, 2] {
                        version = Some(HeaderVersion::PyIPV8Header);
                        break;
                    }

                    // FUTURE: Keep reading more bytes for larger headers until all options are exhausted
                    //
                    // version_bytes.push(seq.next_element()?.ok_or(serde::de::Error::custom("No valid header type could be determined"))?);
                    // version_bytes.push(seq.next_element()?.ok_or(serde::de::Error::custom("No valid header type could be determined"))?);
                    //
                    // add checks for larger headers here

                    // Error when all header bytes are checked
                    return Err(serde::de::Error::custom(
                        "No valid header type could be determined",
                    ));
                }

                match version {
                    Some(i) => match i {
                        HeaderVersion::PyIPV8Header => {
                            let mut mid_hash: [u8; 20] = [0; 20]; // Init with zeroes

                            for i in mid_hash.iter_mut() {
                                *i = seq.next_element()?.ok_or_else(|| {
                                    serde::de::Error::custom(
                                        "No valid header type could be determined",
                                    )
                                })?;
                            }
                            let message_type: u8 = seq.next_element()?.ok_or_else(|| {
                                serde::de::Error::custom("No valid header type could be determined")
                            })?;

                            Ok(Header::py_ipv8_header(mid_hash, message_type))
                        }
                    },
                    None => Err(serde::de::Error::custom(
                        "Somehow the header type was valid but the version None",
                    )),
                }
            }
        }

        Ok(deserializer.deserialize_tuple(std::usize::MAX, HeaderVisitor)?)
    }
}

#[cfg(test)]
macro_rules! create_test_header {
    () => {
        crate::serialization::header::Header {
            size: 23,
            version: crate::serialization::header::HeaderVersion::PyIPV8Header,
            mid_hash: Some(vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            message_type: Some(42),
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
            [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            ],
            42,
        );

        assert_eq!(
            h,
            bincode::config()
                .big_endian()
                .deserialize(&bincode::config().big_endian().serialize(&h).unwrap())
                .unwrap()
        );
    }
}
