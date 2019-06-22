//! This module contains all signature related crypto

use ring;
use untrusted::Input;
use crate::serialization::Packet;
use std::error::Error;
use serde::{Serialize, Serializer, ser::SerializeTuple};
use zerocopy::{AsBytes, FromBytes};
use crate::payloads::Ipv8Payload;
use ring::signature::KeyPair as RingKeyPair;

create_error!(
    KeyRejectedError,
    "During the creation of a keypair, an error occurred. The bytes given are not a valid key."
);
create_error!(
    KeyGenerationError,
    "During the generation of a keypair, an error occurred. The key could not be generated."
);
create_error!(SigningError, "During the signing, a problem occurred.");

/// Type representing a public key. Just a 32 byte array under the hood.
pub type Ed25519PublicKey = [u8; 32];

/// Wrapper struct containing an Ed25519 Key Pair
pub struct KeyPair(pub ring::signature::Ed25519KeyPair);

#[derive(FromBytes, AsBytes)]
#[repr(transparent)]
/// A struct wrapping the 64 bytes of a Signature
pub struct Signature(pub [u8; 64]);

impl Signature {
    /// A constant defining the length of an Ed25519 signature
    pub const ED25519_SIGNATURE_BYTES: usize = 64;
}

impl Ipv8Payload for Signature {}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_tuple(Self::ED25519_SIGNATURE_BYTES)?;
        for i in self.0.iter() {
            state.serialize_element(i)?;
        }
        state.end()
    }
}

impl KeyPair {
    /// Generates a new random keypair
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .or_else(|_| Err(Box::new(KeyGenerationError)))?;

        KeyPair::from_bytes(pkcs8_bytes.as_ref())
    }

    /// Constructs a keypair from a bytearray containing data in
    /// the pkcs8 format
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let trusted_bytes = untrusted::Input::from(bytes);
        let ring_key = ring::signature::Ed25519KeyPair::from_pkcs8(trusted_bytes)
            .or_else(|_| Err(Box::new(KeyRejectedError)))?;
        Ok(KeyPair(ring_key))
    }

    /// Creates a keypair with the provided and seed and checks if the generated key matches the given public key
    pub fn from_seed_checked(
        seed: &[u8; 32],
        publickey: &Ed25519PublicKey,
    ) -> Result<Self, Box<dyn Error>> {
        let trusted_seed = untrusted::Input::from(seed);
        let public = untrusted::Input::from(&*publickey);
        let ring_key =
            ring::signature::Ed25519KeyPair::from_seed_and_public_key(trusted_seed, public)
                .or_else(|_| Err(Box::new(KeyRejectedError)))?;
        Ok(KeyPair(ring_key))
    }

    #[doc(hidden)]
    pub fn from_seed_unchecked(seed: &[u8; 32]) -> Result<Self, Box<dyn Error>> {
        warn!("DANGER ZONE! Creating seed without checking it against a public key");
        let trusted_seed = untrusted::Input::from(seed);
        let ring_key = ring::signature::Ed25519KeyPair::from_seed_unchecked(trusted_seed)
            .or_else(|_| Err(Box::new(KeyRejectedError)))?;
        Ok(KeyPair(ring_key))
    }

    /// Returns the Public part of the KeyPair
    pub fn public_key(&self) -> Result<Ed25519PublicKey, Box<dyn Error>> {
        let pk = &self.0;
        let pk2 = pk.public_key();
        let key = *zerocopy::LayoutVerified::<_, [u8; 32]>::new(pk2.as_ref())
            .ok_or_else(|| Box::new(KeyRejectedError))?;
        Ok(key)
    }
}

/// Helper method which can be used for signing [Packets](crate::serialization::Packet)
pub fn sign_packet(keypair: &KeyPair, message: &Packet) -> Result<Signature, Box<dyn Error>> {
    sign(&keypair, &message.raw())
}

/// A function which takes in a KeyPair and message and returns only the signature
pub fn sign(keypair: &KeyPair, message: &[u8]) -> Result<Signature, Box<dyn Error>> {
    let signature = keypair.0.sign(message);
    let sig = *zerocopy::LayoutVerified::<_, [u8; 64]>::new(signature.as_ref())
        .ok_or_else(|| Box::new(SigningError))?;
    Ok(Signature(sig))
}

/// Wrapper function for [verify] taking packets as input
pub fn verify_packet(public_key: &Ed25519PublicKey, msg: &Packet, sig: &Signature) -> bool {
    let trusted_public_key = untrusted::Input::from(public_key);
    let trusted_msg = untrusted::Input::from(msg.raw());
    let trusted_sig = untrusted::Input::from(&sig.0);
    verify(trusted_public_key, trusted_msg, trusted_sig)
}

/// Wrapper function for [verify] taking bytes as input
pub fn verify_raw(public_key: &Ed25519PublicKey, msg: &[u8], sig: &[u8]) -> bool {
    let trusted_public_key = untrusted::Input::from(public_key);
    let trusted_msg = untrusted::Input::from(msg);
    let trusted_sig = untrusted::Input::from(sig);
    verify(trusted_public_key, trusted_msg, trusted_sig)
}

/// Ed25519 signature verification function
pub fn verify(public_key: Input, msg: Input, sig: Input) -> bool {
    ring::signature::verify(&ring::signature::ED25519, public_key, msg, sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let kp = KeyPair::new();
        assert!(kp.is_ok());
        let kp2 = KeyPair::new();
        assert!(kp2.is_ok());

        assert_ne!(
            kp.unwrap().0.public_key().as_ref(),
            kp2.unwrap().0.public_key().as_ref()
        )
    }

    #[test]
    fn test_sign_packet() {
        let p = Packet(vec![0, 1, 2, 3, 4]);
        let pk = KeyPair::new().unwrap();
        let sig = sign_packet(&pk, &p).unwrap();
        assert_eq!(Signature::ED25519_SIGNATURE_BYTES, sig.0.len());
    }

    #[test]
    fn test_verify_packet() {
        let p = Packet(vec![0, 1, 2, 3, 4]);
        let pk = KeyPair::new().unwrap();
        let sig = sign_packet(&pk, &p).unwrap();
        assert!(verify_packet(&pk.public_key().unwrap(), &p, &sig))
    }

    #[test]
    fn test_verify_packet_raw() {
        let p = Packet(vec![0, 1, 2, 3, 4]);
        let pk = KeyPair::new().unwrap();
        let sig = sign_packet(&pk, &p).unwrap();
        assert!(verify_raw(&pk.public_key().unwrap(), &*p.0, &sig.0))
    }

    #[test]
    fn test_from_seed_unchecked() {
        let seed = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];

        let pk = KeyPair::from_seed_unchecked(&seed);
        assert!(pk.is_ok());
        let pk2 = KeyPair::from_seed_unchecked(&seed);
        assert!(pk2.is_ok());

        assert_eq!(
            pk.unwrap().0.public_key().as_ref(),
            pk2.unwrap().0.public_key().as_ref()
        )
    }
}
