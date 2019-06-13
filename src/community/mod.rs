use crate::serialization::{Packet, PacketDeserializer};
use crate::serialization::header::Header;
use std::error::Error;
use std::collections::HashMap;
use crate::networking::address::Address;

pub mod peer;

create_error!(
    HeaderUnwrapError,
    "The community experienced an error trying to deserialize the header of a packet"
);
create_error!(MidError, "Failed to get the mid");
create_error!(
    UnknownCommunityError,
    "No community with matching mid found"
);

/// # Example Community
/// This is an example of how to create a community
///
/// _**Note:** Try to avoid the use of .unwrap() in actual production code, this is just an example_
///
///
/// ```
/// use ipv8::community::peer::Peer;
/// use ipv8::community::Community;
/// use ipv8::serialization::header::Header;
/// use ipv8::serialization::{PacketDeserializer, Packet};
/// use std::net::Ipv4Addr;
/// use ipv8::networking::address::Address;
/// use std::error::Error;
/// use ipv8::IPv8;
/// use ipv8::configuration::Config;
/// use ipv8::serialization::header::HeaderVersion::PyIPV8Header;
/// use ipv8::crypto::keytypes::PublicKey;
///
/// pub struct TestCommunity{
///     peer: Peer
/// }
///
/// impl TestCommunity{
///     fn new() -> Option<Self> {
///
///         // Use the highest available key
///         let pk: PublicKey = PublicKey::from_vec(vec![
///             48, 129, 167, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 39, 3,
///             129, 146, 0, 4, 2, 86, 251, 75, 206, 159, 133, 120, 63, 176, 235, 178, 14, 8, 197, 59,
///             107, 51, 179, 139, 3, 155, 20, 194, 112, 113, 15, 40, 67, 115, 37, 223, 152, 7, 102,
///             154, 214, 90, 110, 180, 226, 5, 190, 99, 163, 54, 116, 173, 121, 40, 80, 129, 142, 82,
///             118, 154, 96, 127, 164, 248, 217, 91, 13, 80, 91, 94, 210, 16, 110, 108, 41, 57, 4,
///             243, 49, 52, 194, 254, 130, 98, 229, 50, 84, 21, 206, 134, 223, 157, 189, 133, 50, 210,
///             181, 93, 229, 32, 179, 228, 179, 132, 143, 147, 96, 207, 68, 48, 184, 160, 47, 227, 70,
///             147, 23, 159, 213, 105, 134, 60, 211, 226, 8, 235, 186, 20, 241, 85, 170, 4, 3, 40,
///             183, 98, 103, 80, 164, 128, 87, 205, 101, 67, 254, 83, 142, 133,
///         ])?;
///
///         // Actually create the community
///         Some(TestCommunity {
///             peer: Peer::new(
///                 pk,
///                 Address{
///                     address: Ipv4Addr::new(0,0,0,0),
///                     port: 0
///                 },
///                 true,
///             )
///         })
///     }
/// }
///
/// impl Community for TestCommunity{
///
///     // Returns the hash of our master peer
///     fn get_mid(&self) -> Option<Vec<u8>> {
///         Some(self.peer.get_sha1()?.to_vec())
///     }
///
///     // The function which will be called when the community receives a packet
///     fn on_receive(&self, header: Header, deserializer: PacketDeserializer, address: Address) -> Result<(),Box<dyn Error>>{
///         # assert_eq!(header.mid_hash, self.get_mid());
///         # assert_eq!(header.version, PyIPV8Header);
///         # assert_eq!(header.message_type, Some(42));
///         // Do some stuff here like to distribute the message based on it's message_type (in the header)
///         // and check it's signature
///         Ok(())
///     }
/// }
///
/// let mut config = Config::default();
/// let community = TestCommunity::new().unwrap();
/// let mid = community.get_mid();
/// config.communities.add_community(Box::new(community));
/// let ipv8 = IPv8::new(config).unwrap();
///
/// // now simulate a packet coming in
///
/// // Create a packet to test the community with
/// let packet = Packet::new(Header{
///     size: 23,
///     version: PyIPV8Header,
///     mid_hash: mid,
///     message_type: Some(42),
/// }).unwrap();
///
/// // Normally you would want to sign the packet here
///
/// // Send the packet
/// ipv8.config.communities.forward_message(packet,Address{
///     address: Ipv4Addr::new(42,42,42,42),
///     port: 42,
/// });
///
/// ```
pub trait Community {
    /// Returns the hash of our master peer public key
    fn get_mid(&self) -> Option<Vec<u8>>;

    /// Gets called whenever a packet is received directed at this community
    /// DO NOT OVERRIDE
    #[doc(hidden)]
    fn receive(
        &self,
        header: Header,
        deserializer: PacketDeserializer,
        address: Address,
    ) -> Result<(), Box<dyn Error>> {
        // DO NOT OVERRIDE
        //! used to pre-decode the header and filter out messages

        fn warn_deprecated(message: &str, address: Address) -> Result<(), Box<dyn Error>> {
            warn!(
                "Received deprecated message {} from ({:?})",
                message, address
            );
            Ok(())
        }
        match header.message_type.ok_or(HeaderUnwrapError)? {
            255 => warn_deprecated("reserved-255", address),
            254 => warn_deprecated("on-missing-sequence", address),
            253 => warn_deprecated("missing-proof", address),
            252 => warn_deprecated("signature-request", address),
            251 => warn_deprecated("signature-response", address),
            248 => warn_deprecated("on-identity", address),
            247 => warn_deprecated("on-missing-identity", address),
            244 => warn_deprecated("destroy-community", address),
            243 => warn_deprecated("authorize", address),
            242 => warn_deprecated("revoke", address),
            241 => warn_deprecated("subjective-set", address),
            240 => warn_deprecated("missing-subjective-set", address),
            239 => warn_deprecated("on-missing-message", address),
            238 => warn_deprecated("undo-own", address),
            237 => warn_deprecated("undo-other", address),
            236 => warn_deprecated("dynamic-settings", address),
            235 => warn_deprecated("missing-last-message", address),
            _ => self.on_receive(header, deserializer, address),
        }
    }

    fn on_receive(
        &self,
        header: Header,
        deserializer: PacketDeserializer,
        address: Address,
    ) -> Result<(), Box<dyn Error>>;
}

pub struct CommunityRegistry {
    // mid, community
    communities: HashMap<Vec<u8>, Box<dyn Community>>,
}

impl CommunityRegistry {
    pub fn add_community(&mut self, item: Box<dyn Community>) -> Result<(), Box<dyn Error>> {
        match self
            .communities
            .insert(item.get_mid().ok_or(MidError)?, item)
        {
            // none means the key wasn't already present in the map, some means it was and it returns it.
            // We don't care about this.
            _ => Ok(()),
        }
    }

    /// Forwards the message to the corresponding community
    pub fn forward_message(&self, packet: Packet, address: Address) -> Result<(), Box<dyn Error>> {
        // deserialize the header
        let deserializer = packet.start_deserialize();

        // We use peek here instead of get, even though we give the header along with the receive call
        // this is because at this point, the header is not verified yet so we still assume the message is valid.
        // We can't verify the header here yet as not all messages have a signature. Communities will have to decide
        // on their own if they want to verify the header. We do give it along as only having to deserialize the header once
        // makes it slightly more efficient.
        let header = deserializer.peek_header()?;

        // get the mid from the header and use it for a hashtable lookup
        let mid = header.mid_hash.as_ref().ok_or(MidError)?;
        let community = self.communities.get(mid).ok_or(UnknownCommunityError)?;

        // Actually forward it
        community.receive(header, deserializer, address)
    }
}

impl Default for CommunityRegistry {
    fn default() -> Self {
        Self {
            communities: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_networking() {
        use crate::community::peer::Peer;
        use crate::community::Community;
        use crate::serialization::header::Header;
        use crate::serialization::{PacketDeserializer, Packet};
        use std::net::Ipv4Addr;
        use crate::networking::address::Address;
        use std::error::Error;
        use crate::IPv8;
        use crate::configuration::Config;
        use crate::serialization::header::HeaderVersion::PyIPV8Header;
        use crate::crypto::keytypes::PublicKey;

        pub struct TestCommunity {
            peer: Peer,
        }

        impl TestCommunity {
            fn new() -> Option<Self> {
                // Use the highest available key
                let pk: PublicKey = PublicKey::from_vec(vec![
                    48, 129, 167, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0,
                    39, 3, 129, 146, 0, 4, 2, 86, 251, 75, 206, 159, 133, 120, 63, 176, 235, 178,
                    14, 8, 197, 59, 107, 51, 179, 139, 3, 155, 20, 194, 112, 113, 15, 40, 67, 115,
                    37, 223, 152, 7, 102, 154, 214, 90, 110, 180, 226, 5, 190, 99, 163, 54, 116,
                    173, 121, 40, 80, 129, 142, 82, 118, 154, 96, 127, 164, 248, 217, 91, 13, 80,
                    91, 94, 210, 16, 110, 108, 41, 57, 4, 243, 49, 52, 194, 254, 130, 98, 229, 50,
                    84, 21, 206, 134, 223, 157, 189, 133, 50, 210, 181, 93, 229, 32, 179, 228, 179,
                    132, 143, 147, 96, 207, 68, 48, 184, 160, 47, 227, 70, 147, 23, 159, 213, 105,
                    134, 60, 211, 226, 8, 235, 186, 20, 241, 85, 170, 4, 3, 40, 183, 98, 103, 80,
                    164, 128, 87, 205, 101, 67, 254, 83, 142, 133,
                ])?;
                // Actually create the community
                Some(TestCommunity {
                    peer: Peer::new(
                        pk,
                        Address {
                            address: Ipv4Addr::new(0, 0, 0, 0),
                            port: 0,
                        },
                        true,
                    ),
                })
            }
        }

        impl Community for TestCommunity {
            // Returns the hash of our master peer
            fn get_mid(&self) -> Option<Vec<u8>> {
                Some(self.peer.get_sha1()?.to_vec())
            }

            // The function which will be called when the community receives a packet
            fn on_receive(
                &self,
                header: Header,
                deserializer: PacketDeserializer,
                _address: Address,
            ) -> Result<(), Box<dyn Error>> {
                assert_eq!(header.mid_hash, self.get_mid());
                assert_eq!(header.version, PyIPV8Header);
                assert_eq!(header.message_type, Some(42));
                Ok(())
            }
        }

        let mut config = Config::default();
        let community = TestCommunity::new().unwrap();
        let mid = community.get_mid();

        config
            .communities
            .add_community(Box::new(community))
            .unwrap();

        let ipv8 = IPv8::new(config).unwrap();

        // now simulate a packet coming in
        // Create a packet to test the community with
        let packet = Packet::new(Header {
            size: 23,
            version: PyIPV8Header,
            mid_hash: mid,
            message_type: Some(42),
        })
        .unwrap();

        // Normally you would want to sign the packet here

        // Send the packet
        ipv8.config
            .communities
            .forward_message(
                packet,
                Address {
                    address: Ipv4Addr::new(42, 42, 42, 42),
                    port: 42,
                },
            )
            .unwrap();
    }
}
