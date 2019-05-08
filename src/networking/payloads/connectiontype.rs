#[derive(Debug, PartialEq)]
pub enum ConnectionType {
  UNKNOWN,
  PUBLIC,
  SYMMETRICNAT,
}

impl ConnectionType {
  pub fn encode(&self) -> (bool, bool) {
    match self {
      ConnectionType::UNKNOWN => (false, false),
      ConnectionType::PUBLIC => (true, false),
      ConnectionType::SYMMETRICNAT => (true, true),
    }
  }

  pub fn decode(bits: (bool, bool)) -> Self {
    match bits {
      (false, false) => ConnectionType::UNKNOWN,
      (true, false) => ConnectionType::PUBLIC,
      (false, true) => ConnectionType::UNKNOWN, // not in py-ipv8 but this case is not specified and thus unknown
      (true, true) => ConnectionType::SYMMETRICNAT,
    }
  }
}
