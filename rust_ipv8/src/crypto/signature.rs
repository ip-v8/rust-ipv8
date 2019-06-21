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

pub type Ed25519PublicKey = [u8; 32];

pub struct KeyPair(pub ring::signature::Ed25519KeyPair);

#[derive(FromBytes, AsBytes)]
#[repr(transparent)]
pub struct Signature([u8; 64]);

impl Signature {
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
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .or(Err(Box::new(KeyGenerationError)))?;

        KeyPair::from_bytes(pkcs8_bytes.as_ref())
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let trusted_bytes = untrusted::Input::from(bytes);
        let ring_key = ring::signature::Ed25519KeyPair::from_pkcs8(trusted_bytes)
            .or(Err(Box::new(KeyRejectedError)))?;
        Ok(KeyPair(ring_key))
    }

    #[cfg(test)]
    pub fn from_seed_unchecked(seed: [u8; 32]) -> Result<Self, Box<dyn Error>> {
        let trusted_seed = untrusted::Input::from(&seed);
        let ring_key = ring::signature::Ed25519KeyPair::from_seed_unchecked(trusted_seed)
            .or(Err(Box::new(KeyRejectedError)))?;
        Ok(KeyPair(ring_key))
    }

    pub fn public_key(&self) -> Result<Ed25519PublicKey, Box<dyn Error>> {
        let pk = &self.0;
        let pk2 = pk.public_key();
        let key = *zerocopy::LayoutVerified::<_, [u8; 32]>::new(pk2.as_ref())
            .ok_or(Box::new(KeyRejectedError))?;
        Ok(key)
    }
}

pub fn sign_packet(keypair: KeyPair, message: &Packet) -> Result<Signature, Box<dyn Error>> {
    sign(keypair, &message.raw())
}

pub fn sign(keypair: KeyPair, message: &[u8]) -> Result<Signature, Box<dyn Error>> {
    let signature = keypair.0.sign(message);
    let sig = *zerocopy::LayoutVerified::<_, [u8; 64]>::new(signature.as_ref())
        .ok_or(Box::new(SigningError))?;
    Ok(Signature(sig))
}

/// Wrapper function for [`verify`] taking bytes as input
pub fn verify_packet(public_key: &Ed25519PublicKey, msg: &Packet, sig: &[u8]) -> bool {
    let trusted_public_key = untrusted::Input::from(public_key);
    let trusted_msg = untrusted::Input::from(msg.raw());
    let trusted_sig = untrusted::Input::from(sig);
    verify(trusted_public_key, trusted_msg, trusted_sig)
}

/// ed25519 signature verification function
pub fn verify(public_key: Input, msg: Input, sig: Input) -> bool {
    ring::signature::verify(&ring::signature::ED25519, public_key, msg, sig).is_ok()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_sign() {}
}
