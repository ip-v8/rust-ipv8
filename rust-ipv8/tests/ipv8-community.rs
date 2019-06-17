use ipv8::networking::NetworkSender;

#[test]
fn community_integration_test() {
    use ipv8::community::peer::Peer;
    use ipv8::community::Community;
    use ipv8::serialization::header::Header;
    use ipv8::serialization::{PacketDeserializer, Packet};
    use std::net::{Ipv4Addr, SocketAddr, IpAddr};
    use ipv8::networking::address::Address;
    use std::error::Error;
    use ipv8::IPv8;
    use ipv8::configuration::Config;
    use ipv8::serialization::header::HeaderVersion::PyIPV8Header;
    use ipv8::crypto::keytypes::PublicKey;

    pub struct TestCommunity {
        peer: Peer,
    }

    impl TestCommunity {}

    impl Community for TestCommunity {
        fn new(endpoint: &NetworkSender) -> Result<Self, Box<dyn Error>> {
            // Use the highest available key
            let pk: PublicKey = PublicKey::from_vec(vec![
                48, 129, 167, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 39, 3,
                129, 146, 0, 4, 2, 86, 251, 75, 206, 159, 133, 120, 63, 176, 235, 178, 14, 8, 197,
                59, 107, 51, 179, 139, 3, 155, 20, 194, 112, 113, 15, 40, 67, 115, 37, 223, 152, 7,
                102, 154, 214, 90, 110, 180, 226, 5, 190, 99, 163, 54, 116, 173, 121, 40, 80, 129,
                142, 82, 118, 154, 96, 127, 164, 248, 217, 91, 13, 80, 91, 94, 210, 16, 110, 108,
                41, 57, 4, 243, 49, 52, 194, 254, 130, 98, 229, 50, 84, 21, 206, 134, 223, 157,
                189, 133, 50, 210, 181, 93, 229, 32, 179, 228, 179, 132, 143, 147, 96, 207, 68, 48,
                184, 160, 47, 227, 70, 147, 23, 159, 213, 105, 134, 60, 211, 226, 8, 235, 186, 20,
                241, 85, 170, 4, 3, 40, 183, 98, 103, 80, 164, 128, 87, 205, 101, 67, 254, 83, 142,
                133,
            ])?;

            // Actually create the community
            Ok(TestCommunity {
                peer: Peer::new(
                    pk,
                    Address(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 42)),
                    true,
                ),
            })
        }

        // Returns the hash of our master peer
        fn get_mid(&self) -> Option<Vec<u8>> {
            Some(self.peer.get_sha1()?.to_vec())
        }

        // The function which will be called when the community receives a packet
        fn on_receive(
            &self,
            header: Header,
            deserializer: PacketDeserializer,
            address: Address,
        ) -> Result<(), Box<dyn Error>> {
            assert_eq!(header.mid_hash, self.get_mid());
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
        mid_hash: mid,
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
