use std::net::Ipv4Addr;

/// IPV4 address
#[derive(Debug, PartialEq)]
pub struct Address {
  pub address: Ipv4Addr,
  pub port: u16,
}

impl Address {

}

impl std::fmt::Display for Address {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Ipv4Addr: ({}, {})", self.address, self.port)
  }

}

