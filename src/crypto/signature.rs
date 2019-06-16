use std::error::Error;

use serde::ser::SerializeTuple;
use serde::ser::Serializer;
use serde::Serialize;

use crate::crypto::keytypes::{PrivateKey, PublicKey};
use crate::crypto::{
    create_signature_ed25519, verify_signature_ed25519
};
use crate::payloads::Ipv8Payload;

create_error!(KeyError, "Invalid Key");
create_error!(CurveError, "This curve is unknown");

/// A struct containing a cryptographic signature
#[derive(PartialEq, Debug)]
pub struct Signature {
    pub signature: Vec<u8>,
}

impl Ipv8Payload for Signature {
    // this is just to allow it to be serialized to a packet. It isn't actually a "payload"
}

/// Make the signature serializable
impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(self.signature.len())?;
        for i in &self.signature {
            state.serialize_element(i)?;
        }
        state.end()
    }
}

impl Signature {
    /// Signature can be created from its binary string (bytes)
    pub fn from_bytes(data: &[u8], skey: &PrivateKey) -> Result<Self, Box<dyn Error>> {
        // skey.1 is the verification key
        let signature: Vec<u8> = create_signature_ed25519(data, &skey.1)?.as_ref().to_owned();
        Ok(Self { signature })
    }

    /// Verify given data with this signature
    pub fn verify(&self, data: &[u8], pkey: PublicKey) -> bool {
        match pkey {
            PublicKey(_, key_verification) => {
                match verify_signature_ed25519(self.signature.to_owned(), data, key_verification) {
                    Ok(i) => i,
                    Err(_) => false, // if an error occurred, the signature is invalid and therefore did not match
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    #![allow(non_snake_case)]

    use super::*;

    use rust_sodium::crypto::sign::ed25519;

    #[test]
    pub fn test_signature_ed25519() {
        let seed = ed25519::Seed::from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (pkey, skey) = ed25519::keypair_from_seed(&seed);

        let seed = ed25519::Seed::from_slice(&[
            1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (e_pkey, e_skey) = ed25519::keypair_from_seed(&seed);

        assert_ne!(e_pkey, pkey);
        assert_ne!(e_skey, skey);

        let sig = Signature::from_bytes(&[42, 43, 44], &PrivateKey(e_skey, skey)).unwrap();
        assert_eq!(
            vec![
                31, 14, 50, 234, 129, 186, 124, 84, 223, 67, 233, 173, 116, 95, 218, 136, 149, 223,
                171, 234, 13, 173, 164, 78, 74, 59, 106, 31, 252, 230, 79, 207, 199, 207, 134, 92,
                252, 211, 142, 172, 183, 61, 17, 236, 208, 124, 206, 37, 204, 85, 62, 155, 171,
                129, 153, 90, 3, 148, 202, 220, 53, 159, 172, 7
            ],
            sig.signature
        );

        assert!(sig.verify(&[42, 43, 44], PublicKey(pkey, pkey)));
    }
}
