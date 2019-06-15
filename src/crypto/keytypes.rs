//! Module for managing public and private keys

use std::fmt;
use sha1::{Sha1, Digest};
use rust_sodium::crypto::sign::ed25519;
use std::error::Error;

create_error!(KeyCreationError, "Error creating a Key");
create_error!(UnsupportedKeyTypeError, "Error KeyType is not supported");

pub const ED25519_SIZE: usize = 64;

/// Struct containing the public key.
pub struct PublicKey(pub ed25519::PublicKey, pub ed25519::PublicKey);
pub struct PrivateKey(pub ed25519::SecretKey, pub ed25519::SecretKey);

impl PublicKey {
    pub fn to_vec(&self) -> Option<Vec<u8>> {
        Some(match self {
            PublicKey(key_encryption, key_verification) => {
                //translates to "LibNaCLPK:" which is the (very silly) prefix used by py-ipv8
                let mut res = vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58];
                res.extend_from_slice(key_encryption.as_ref());
                res.extend_from_slice(key_verification.as_ref());
                res
            }
        })
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        // literally "LibNaCLPK:"
        let ed25519prefix = &[76, 105, 98, 78, 97, 67, 76, 80, 75, 58];

        if data.starts_with(ed25519prefix) {
            // libnacl
            // divide by two to get the cutoff point for the two keys (encryption,verify)
            let key_length = (data.len() - ed25519prefix.len()) / 2;

            let key_encryption = match ed25519::PublicKey::from_slice(
                &data[ed25519prefix.len()..ed25519prefix.len() + key_length],
            ) {
                Some(k) => k,
                None => return Err(Box::new(KeyCreationError)),
            };

            let key_verification =
                ed25519::PublicKey::from_slice(&data[ed25519prefix.len() + key_length as usize..])
                    .ok_or(KeyCreationError)?;

            Ok(PublicKey(key_encryption, key_verification))
        } else {
            Err(Box::new(UnsupportedKeyTypeError))
        }
    }

    pub fn sha1(&self) -> (Vec<u8>, Vec<u8>) {
        // Hash left key
        let mut hasher = Sha1::new();
        hasher.input(&self.0);
        let left = hasher.result().to_vec();

        // Hash right key
        let mut hasher = Sha1::new();
        hasher.input(&self.1);
        let right = hasher.result().to_vec();

        // Return
        (left, right)
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.to_vec() == other.to_vec()
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Public ED25519 Key")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vec_ed25519() {
        let seed = ed25519::Seed::from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (e_pkey, _) = ed25519::keypair_from_seed(&seed);

        let seed = ed25519::Seed::from_slice(&[
            1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (s_pkey, _) = ed25519::keypair_from_seed(&seed);

        let mut keyvec = vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58]; // libnaclPK:
        keyvec.extend_from_slice(&e_pkey.as_ref()); // Encryption key
        keyvec.extend_from_slice(&s_pkey.as_ref()); // Verification key

        match PublicKey::from_vec(keyvec).unwrap() {
            PublicKey(key_enc, key_verification) => {
                assert_eq!(key_enc, e_pkey);
                assert_eq!(key_verification, s_pkey);
            }
        }
    }

    #[test]
    fn test_to_vec_ed25519() {
        let seed = ed25519::Seed::from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (pkey, _) = ed25519::keypair_from_seed(&seed);
        let mut keyvec = vec![76, 105, 98, 78, 97, 67, 76, 80, 75, 58]; // libnaclPK:
        keyvec.extend_from_slice(&pkey.as_ref());
        keyvec.extend_from_slice(&pkey.as_ref());
        assert_eq!(
            PublicKey::from_vec(keyvec.clone())
                .unwrap()
                .to_vec()
                .unwrap(),
            keyvec
        );
    }
}
