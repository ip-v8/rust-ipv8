pub mod signaturepayload;

use rust_sodium::crypto::sign::ed25519;
use bincode::ErrorKind;
use std::fmt;
use crate::networking::crypto::signaturepayload::SignaturePayload;

create_error!(KeyError, "Invalid key provided");
create_error!(SizeError, "Invalid result size");

/// wrapper function for signing data using ed25519
fn create_signature_ed25519(data: &[u8], skey: &[u8]) -> Result<u64, SizeError>{
  let res : &[u8] = ed25519::sign_detached(data,pkey).as_ref();
  if res.len() > 8 {
    return Err(SizeError);
  }
  Ok(u64::from_be_bytes(res))
}

/// wrapper function for verifying data using ed25519
fn verify_signature_ed25519(signature: &[u8], data: &[u8], pkey: &[u8]) -> Result<bool,KeyError>{
  Ok(ed25519::verify_detached(ed25519::Signature::from_slice(signature),data, &match ed25519::PublicKey::from_slice(pkey){
    Some(i) => i,
    None(i) => Err(KeyError)
  }))
}
