use crate::crypto::keytypes::PublicKey;
use crate::networking::address::Address;

pub struct Peer{
    key: PublicKey,
    address: Address,
    intro: bool
}

impl Peer{
    pub fn new(key: PublicKey, address: Address, intro: bool) -> Self{
        Self{key,address,intro}
    }

    pub fn get_sha1(&self) -> Option<[u8; 20]>{
        self.key.sha1()
    }
}
