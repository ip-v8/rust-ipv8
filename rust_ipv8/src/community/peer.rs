use crate::crypto::keytypes::PublicKey;
use crate::networking::address::Address;

pub struct Peer {
    pub key: PublicKey,
    pub address: Address,
    pub intro: bool,
}

impl Peer {
    pub fn new(key: PublicKey, address: Address, intro: bool) -> Self {
        Self {
            key,
            address,
            intro,
        }
    }

    pub fn get_sha1(&self) -> (Vec<u8>, Vec<u8>) {
        self.key.sha1()
    }
}

#[cfg(test)]
mod tests {
    use crate::community::peer::Peer;
    use crate::crypto::keytypes::PublicKey;

    use std::net::{Ipv4Addr, SocketAddr, IpAddr};
    use crate::networking::address::Address;
    use rust_sodium::crypto::sign::ed25519;

    fn get_key() -> PublicKey {
        let seed = ed25519::Seed::from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (pkey1, _) = ed25519::keypair_from_seed(&seed);
        let (pkey2, _) = ed25519::keypair_from_seed(&seed);
        PublicKey(pkey1, pkey2)
    }

    fn get_addr() -> Address {
        Address(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(42, 42, 42, 42)),
            8000,
        ))
    }

    #[test]
    fn contructor_test() {
        let peer = Peer::new(get_key(), get_addr(), true);

        assert_eq!(get_key(), peer.key);
        assert_eq!(get_addr(), peer.address);
        assert_eq!(true, peer.intro);
    }
}
