//! Represents the type of connection two communities have.

#[derive(Debug, PartialEq)]
/// Sent as a member of a number of payloads like the [IntroductionRequestPayload](crate::payloads::introductionrequestpayload::IntroductionRequestPayload), in their flags fields.
///
/// **_NOTE_**: with IPv6 support, another connection type might be added to signify this.
pub enum ConnectionType {
    /// A Public connnection meaning: Easily reachable and no NAT Puncturing needed.
    PUBLIC,
    /// The ConnectionType specifiying both parties being behind a NAT used to indicate NAT puncturing is required.
    SYMMETRICNAT,
    /// Fallback if connectiontype could not be determined or is not known.
    UNKNOWN,
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
