//! Module containing structs for working with network addresses
//!
//! This also provides the serialization and deserialization of network addresses
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::net::{Ipv4Addr, SocketAddr, IpAddr};
use serde::ser::SerializeTuple;
use serde::de::{Visitor, SeqAccess};
use std::fmt;

/// Wrapper for a SocketAddr. Had to be wrapped to serialize it properly
/// Currently this can only be used as an IPV4 address + port
#[derive(Debug, PartialEq)]
pub struct Address(pub SocketAddr);

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v4 = match self.0.ip() {
            IpAddr::V4(a) => a,
            IpAddr::V6(_) => {
                return Err(serde::ser::Error::custom(
                    "ip-v8 does not (yet) support ipv6",
                ))
            }
        };

        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&v4)?;
        state.serialize_element(&self.0.port())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Address {
    /// deserializes a Header
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // first deserialize it to a temporary struct which literally represents the packer
        #[doc(hidden)]
        struct AddressVistor;
        impl<'de> Visitor<'de> for AddressVistor {
            type Value = Address;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Address")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let addr: Ipv4Addr = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Could not deserialize the address"))?;
                let port: u16 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("Could not deserialize the port"))?;

                Ok(Address(SocketAddr::new(IpAddr::V4(addr), port)))
            }
        }

        Ok(deserializer.deserialize_tuple(2, AddressVistor)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode;
    use std::net::{SocketAddr, IpAddr};

    #[test]
    fn test_serialization() {
        let i = Address(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8000,
        ));

        assert_eq!(bincode::serialize(&i).unwrap(), vec![127, 0, 0, 1, 64, 31]);
    }

    #[test]
    fn test_deserialization() {
        let i = Address(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            4242,
        ));

        assert_eq!(
            i,
            bincode::deserialize(&bincode::serialize(&i).unwrap()).unwrap()
        );
    }
}
