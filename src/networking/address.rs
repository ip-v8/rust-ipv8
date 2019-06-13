use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// IPV4 address
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub address: Ipv4Addr,
    pub port: u16,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Ipv4Addr: ({}, {})", self.address, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode;

    #[test]
    fn test_serialization() {
        let i = Address {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 8000,
        };

        assert_eq!(bincode::serialize(&i).unwrap(), vec![127, 0, 0, 1, 64, 31]);
    }

    #[test]
    fn test_deserialization() {
        let i = Address {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 8000,
        };
        assert_eq!(
            i,
            bincode::deserialize(&bincode::serialize(&i).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_fmt() {
        let i = Address {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 8000,
        };

        assert_eq!("Ipv4Addr: (127.0.0.1, 8000)", format!("{}", i))
    }
}
