use crate::serialization::{Packet, PacketDeserializer};
use crate::serialization::header::Header;
use std::error::Error;
use std::collections::HashMap;
use crate::networking::address::Address;
use crate::networking::NetworkSender;

#[cfg(test)]
use std::sync::atomic::AtomicUsize;

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

#[cfg(test)]
static WARN_DEPRECATED_CALLS: AtomicUsize = AtomicUsize::new(0);

/// # Community struct
/// This is the main struct defining a community
///
/// ## Example Community
/// This is an example of how to create a community
///
/// _**Note:** Try to avoid the use of .unwrap() in actual production code, this is just an example_
///
pub trait Community {
    fn new(endpoint: &NetworkSender) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    /// Returns a unique (currently 20 byte) sequence identifying a community.
    ///
    /// This is used to be the SHA1 hash of its public key. You are free to choose whatever.
    ///
    /// As OpenSSL keys are deprecated, this library provides no way of calculating the sha1 of an OpenSSL key.
    /// Master peer keys still can be OpenSSL keys. This SHA1 has to be hardcoded for communities that are
    /// compatible with old communities. New communities recommended to use ED25519 in the future which
    /// sha1 hashes can be calculated.
    ///
    /// The sha1 of a key does not serve any purpose besides uniquely identifying communities and as such can be any
    /// unique 20 byte sequence.
    fn get_mid(&self) -> Vec<u8>;

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
        //!

        // Used in on_receive. Has to be outside to be testable.
        #[doc(hidden)]
        fn warn_deprecated(message: &str, address: Address) -> Result<(), Box<dyn Error>> {
            warn!(
                "Received deprecated message {} from ({:?})",
                message, address
            );

            #[cfg(test)]
            {
                WARN_DEPRECATED_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }

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

    /// This method called for every incoming message, directed at this community, which is not captured.
    ///
    /// Messages are captured whenever they have a reserved message_type (235 ~ 255). These are used for legacy support
    /// and some default responses which every community should give.
    fn on_receive(
        &self,
        header: Header,
        deserializer: PacketDeserializer,
        address: Address,
    ) -> Result<(), Box<dyn Error>>;
}

/// Every different kind of community is registered here with it's MID.
///
/// So that incoming messages can be distributed to the right communities. Makes use of a hashmap to achieve
/// O(1) lookup time.
pub struct CommunityRegistry {
    // mid, community
    #[cfg(test)]
    pub communities: HashMap<Vec<u8>, Box<dyn Community>>,
    #[cfg(not(test))]
    communities: HashMap<Vec<u8>, Box<dyn Community>>,
}

impl CommunityRegistry {
    /// Adds a community to the registry.
    pub fn add_community(&mut self, item: Box<dyn Community>) -> Result<(), Box<dyn Error>> {
        match self.communities.insert(item.get_mid(), item) {
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
    /// Returns a new community registry with all the built-in communities already registered.
    /// All custom communities can be added with the [add_community](#method.add_community) method.
    fn default() -> Self {
        Self {
            communities: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::networking::address::Address;
    use std::net::{SocketAddr, IpAddr, SocketAddrV4};
    use std::error::Error;
    use crate::networking::NetworkSender;
    use crate::community::peer::Peer;
    use crate::community::{Community, CommunityRegistry};
    use crate::serialization::header::Header;
    use crate::serialization::{PacketDeserializer, Packet};
    use std::net::Ipv4Addr;
    use crate::IPv8;
    use crate::configuration::Config;
    use crate::serialization::header::HeaderVersion::PyIPV8Header;
    use crate::crypto::keytypes::PublicKey;
    use std::sync::atomic::Ordering;
    use crate::networking::test_helper::localhost;
    use rust_sodium::crypto::sign::ed25519;

    pub struct TestCommunity {
        peer: Peer,
    }

    impl Community for TestCommunity {
        fn new(endpoint: &NetworkSender) -> Result<Self, Box<dyn Error>> {
            // Use the highest available key
            let seed = ed25519::Seed::from_slice(&[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ])
            .unwrap();
            let (pkey1, _) = ed25519::keypair_from_seed(&seed);
            let (pkey2, _) = ed25519::keypair_from_seed(&seed);
            let pk = PublicKey(pkey1, pkey2);
            // Actually create the community
            Ok(TestCommunity {
                peer: Peer::new(
                    pk,
                    Address(SocketAddr::new(
                        IpAddr::V4(Ipv4Addr::new(42, 42, 42, 42)),
                        8000,
                    )),
                    true,
                ),
            })
        }

        // Returns the hash of our master peer
        fn get_mid(&self) -> Vec<u8> {
            self.peer.get_sha1().0
        }
      
        // The function which will be called when the community receives a packet
        fn on_receive(
            &self,
            header: Header,
            deserializer: PacketDeserializer,
            _address: Address,
        ) -> Result<(), Box<dyn Error>> {

            assert_eq!(header.mid_hash.unwrap(), self.get_mid());
            assert_eq!(header.version, PyIPV8Header);
            assert_eq!(header.message_type, Some(42));
            Ok(())
        }
    }

    #[test]
    fn test_deprecated() {
        let mut config = Config::default();
        config.sending_address = Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0));
        config.receiving_address =
            Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0));

        let ipv8 = IPv8::new(config).unwrap();

        let community = TestCommunity::new(&ipv8.network_sender).unwrap();
        for i in &[
            255, 254, 253, 252, 251, 248, 247, 244, 243, 242, 241, 240, 239, 238, 237, 236, 235,
        ] {
            let address = Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0));

            let packet = Packet::new(Header {
                size: 23,
                version: PyIPV8Header,
                mid_hash: Some(community.get_mid()),
                message_type: Some(*i),
            })
            .unwrap();
            let deser = packet.start_deserialize();
            let header = deser.peek_header().unwrap();
            community.receive(header, deser, address).unwrap();
        }
        assert_eq!(17, WARN_DEPRECATED_CALLS.load(Ordering::SeqCst))
    }

    #[test]
    fn test_add_community() {
        let config = Config::default();
        let ipv8 = IPv8::new(config).unwrap();
        let community = Box::new(TestCommunity::new(&ipv8.network_sender).unwrap());
        let the_same = Box::new(TestCommunity::new(&ipv8.network_sender).unwrap());
        let mid = &*community.get_mid();
        let mut registry: CommunityRegistry = CommunityRegistry::default();

        registry.add_community(community).unwrap();

        let get = registry.communities.get(mid).unwrap();

        assert_eq!(the_same.get_mid(), get.get_mid()); // TODO: More thorough comparison
    }

    #[test]
    fn test_networking() {
        let mut config = Config::default();
        config.receiving_address = localhost();
        config.sending_address = localhost();
        config.buffersize = 2048;

        let mut ipv8 = IPv8::new(config).unwrap();

        let community = TestCommunity::new(&ipv8.network_sender).unwrap();
        let mid = community.get_mid();

        ipv8.communities.add_community(Box::new(community)).unwrap();

        // now simulate a packet coming in
        // Create a packet to test the community with
        let packet = Packet::new(Header {
            size: 23,
            version: PyIPV8Header,
            mid_hash: Some(mid),
            message_type: Some(42),
        })
        .unwrap();

        // Normally you would want to sign the packet here

        // Send the packet
        ipv8.communities
            .forward_message(
                packet,
                Address(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(42, 42, 42, 42)),
                    42,
                )),
            )
            .unwrap();
    }
}
