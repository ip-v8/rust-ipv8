#[derive(Debug, PartialEq)]
pub enum ConnectionType {
    UNKNOWN,
    PUBLIC,
    SYMMETRICNAT,
}

impl ConnectionType {
    /// encodes a connection type into 2 booleans.
    /// Combinations are chosen semi-arbitrarily but are standardized. Booleans are used to go into a BITS field in a packet.
    pub fn encode(&self) -> (bool, bool) {
        match self {
            ConnectionType::UNKNOWN => (false, false),
            ConnectionType::PUBLIC => (true, false),
            ConnectionType::SYMMETRICNAT => (true, true),
        }
    }

    /// Decodes two booleans into the corresponding connection type as encoded with the above encode method.
    pub fn decode(bits: (bool, bool)) -> Self {
        match bits {
            (false, false) => ConnectionType::UNKNOWN,
            (true, false) => ConnectionType::PUBLIC,
            (false, true) => ConnectionType::UNKNOWN, // not in py-ipv8 but this case is not specified and thus unknown
            (true, true) => ConnectionType::SYMMETRICNAT,
        }
    }
}
