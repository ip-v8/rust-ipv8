use crate::crypto::signature::{Ed25519PublicKey};
use crate::networking::address::Address;
use ring::digest::{digest, SHA1};

pub struct Peer {
    pub key: Ed25519PublicKey,
    pub address: Address,
    pub intro: bool,
}

impl Peer {
    pub fn new(key: Ed25519PublicKey, address: Address, intro: bool) -> Self {
        Self {
            key,
            address,
            intro,
        }
    }

    pub fn get_sha1(&self) -> Vec<u8> {
        digest(&SHA1, &self.key).as_ref().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::community::peer::Peer;

    use std::net::{Ipv4Addr, SocketAddr, IpAddr};
    use crate::networking::address::Address;
    use crate::crypto::signature::KeyPair;

    fn get_key() -> KeyPair {
        KeyPair::from_seed_unchecked([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap()
    }

    fn get_addr() -> Address {
        Address(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(42, 42, 42, 42)),
            8000,
        ))
    }

    #[test]
    fn contructor_test() {
        let peer = Peer::new(get_key().public_key().unwrap(), get_addr(), true);

        assert_eq!(get_key().public_key().unwrap(), peer.key);
        assert_eq!(get_addr(), peer.address);
        assert_eq!(true, peer.intro);
    }
}
