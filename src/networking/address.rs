/// IPV4 address
pub struct IPV4Addr {
  pub address: String,
  pub port: u16,
}

impl std::fmt::Display for IPV4Addr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "IPV4Addr: ({}, {})", self.address, self.port)
  }
}

/// IPV6 address
pub struct IPV6Addr {
  pub address: String,
  pub port: u16,
}

impl std::fmt::Display for IPV6Addr {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "IPV6Addr: ({}, {})", self.address, self.port)
  }
}

/// An enum containing both ipv6 and ipv4 addresses
pub enum IpAddress {
  IPV4(IPV4Addr),
  IPV6(IPV6Addr),
}
