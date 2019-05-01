/// An enum containing both ipv6 and ipv4 addresses

#[derive(Debug)]
pub enum IPVersion {
  IPV4,
  IPV6,
}

/// IPV4 address
pub struct IPAddress {
  pub address: String,
  pub port: u16,
  pub version: IPVersion,
}


impl std::fmt::Display for IPAddress {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "IPV6Addr: ({}, {} version {:?})",
      self.address, self.port, self.version
    )
  }
}

