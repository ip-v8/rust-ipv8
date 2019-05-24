pub mod signature;
pub mod keytypes;

use rust_sodium::crypto::sign::ed25519;
use std::error::Error;
use std::fmt;
use openssl::sign::Signer;
use openssl::pkey::{Private, Public};
use openssl::ecdsa::EcdsaSig;
use openssl::bn::BigNum;
use std::os::raw::c_int;

create_error!(SignatureError, "Invalid signature");
create_error!(SizeError, "Invalid input size");
create_error!(OpenSSLError, "OpenSSL had a rapid unscheduled disassembly (oops)");

/// wrapper function for signing data using ed25519
pub fn create_signature_ed25519(data: &[u8], skey: ed25519::SecretKey) -> Result<ed25519::Signature, Box<Error>>{
  Ok(ed25519::sign_detached(data,&skey))
}

/// wrapper function for verifying data using ed25519
pub fn verify_signature_ed25519(signature: Vec<u8>, data: &[u8], pkey: ed25519::PublicKey) -> Result<bool,Box<Error>>{
  Ok(ed25519::verify_detached(&match ed25519::Signature::from_slice(&*signature) {
    Some(i) => i,
    None => return Err(Box::new(SignatureError))
  },data, &pkey))
}

/// wrapper function for signing data using ed25519
pub fn create_signature_openssl(data: &[u8], skey: openssl::ec::EcKey<Private>) -> Result<EcdsaSig, Box<Error>>{
  if data.len() > c_int::max_value() as usize{
    return Err(Box::new(SizeError));
  }
  match EcdsaSig::sign(data, &*skey){
    Ok(i) => Ok(i),
    Err(_) => Err(Box::new(OpenSSLError))
  }
}

/// wrapper function for verifying data using ed25519
pub fn verify_signature_openssl(signature: (BigNum, BigNum), data: &[u8], pkey: openssl::ec::EcKey<Public>) -> Result<bool,Box<Error>>{
  if data.len() > c_int::max_value() as usize{
    return Err(Box::new(SizeError));
  }
  match match EcdsaSig::from_private_components(signature.0,signature.1){
    Ok(i) => i,
    Err(_) => return Err(Box::new(OpenSSLError)) // Should **never** happen but if it does openssl burn it
  }.verify(data,&*pkey) {
    Ok(i) => Ok(i),
    Err(_) => return Err(Box::new(OpenSSLError)) // Should **never** happen but if it does openssl burn it
  }
}
