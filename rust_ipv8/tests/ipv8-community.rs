use rust_ipv8::crypto::signature::KeyPair;
use rust_ipv8::crypto::signature::Ed25519PublicKey;

#[test]
fn community_integration_test() {
    use rust_ipv8::crypto::signature::Ed25519PublicKey;
    use rust_ipv8::community::peer::Peer;
    use rust_ipv8::community::Community;
    use rust_ipv8::serialization::header::Header;
    use rust_ipv8::serialization::{PacketDeserializer, Packet};
    use std::net::{Ipv4Addr, SocketAddr, IpAddr};
    use rust_ipv8::networking::address::Address;
    use std::error::Error;
    use rust_ipv8::IPv8;
    use rust_ipv8::configuration::Config;
    use rust_ipv8::serialization::header::HeaderVersion::PyIPV8Header;
    use rust_ipv8::networking::NetworkSender;

    fn from_seed_unchecked(seed: [u8; 32]) -> Result<KeyPair, Box<dyn Error>> {
        let trusted_seed = untrusted::Input::from(&seed);
        let ring_key = ring::signature::Ed25519KeyPair::from_seed_unchecked(trusted_seed).unwrap();
        Ok(KeyPair(ring_key))
    }

    pub struct TestCommunity {
        peer: Peer,
    }

    impl TestCommunity {}

    impl Community for TestCommunity {
        fn new(endpoint: &NetworkSender) -> Result<Self, Box<dyn Error>> {
            let pk: KeyPair = from_seed_unchecked([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ])
            .unwrap();

            // Actually create the community
            Ok(TestCommunity {
                peer: Peer::new(
                    pk.public_key().unwrap(),
                    Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 42)),
                    true,
                ),
            })
        }

        // Returns the hash of our master peer
        fn get_mid(&self) -> Vec<u8> {
            self.peer.get_sha1()
        }

        // The function which will be called when the community receives a packet
        fn on_receive(
            &self,
            header: Header,
            deserializer: PacketDeserializer,
            address: Address,
        ) -> Result<(), Box<dyn Error>> {
            assert_eq!(header.mid_hash, Some(self.get_mid()));
            assert_eq!(header.version, PyIPV8Header);
            assert_eq!(header.message_type, Some(42));
            // Do some stuff here like to distribute the message based on it's message_type (in the header)
            // and check it's signature
            Ok(())
        }
    }

    let config = Config::default();
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
