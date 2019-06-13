use crate::crypto::keytypes::PublicKey;
use crate::networking::address::Address;

pub struct Peer {
    key: PublicKey,
    address: Address,
    intro: bool,
}

impl Peer {
    pub fn new(key: PublicKey, address: Address, intro: bool) -> Self {
        Self {
            key,
            address,
            intro,
        }
    }

    pub fn get_sha1(&self) -> Option<[u8; 20]> {
        self.key.sha1()
    }
}

#[cfg(test)]
mod tests {
    use crate::community::peer::Peer;
    use crate::crypto::keytypes::PublicKey;
    use crate::networking::address::Address;
    use std::net::Ipv4Addr;

    fn get_key() -> PublicKey {
        let keyvec = vec![
            48, 64, 48, 16, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 5, 43, 129, 4, 0, 1, 3, 44, 0, 4,
            0, 80, 239, 172, 104, 165, 76, 172, 6, 229, 136, 156, 105, 23, 249, 46, 30, 148, 87,
            105, 57, 6, 105, 134, 2, 229, 115, 169, 44, 162, 41, 190, 228, 56, 20, 100, 64, 79,
            167, 224, 118, 14,
        ];
        PublicKey::from_vec(keyvec.clone()).unwrap()
    }

    fn get_addr() -> Address {
        Address {
            address: Ipv4Addr::new(42, 42, 42, 42),
            port: 42,
        }
    }

    #[test]
    fn contructor_test() {
        let peer = Peer::new(get_key(), get_addr(), true);

        assert_eq!(get_key(), peer.key);
        assert_eq!(get_addr(), peer.address);
        assert_eq!(true, peer.intro);
    }

    #[test]
    fn sha1_test() {
        let peer = Peer::new(get_key(), get_addr(), false);
        assert_eq!(get_key().sha1(), peer.get_sha1());
        assert_eq!(false, peer.intro)
    }
}
