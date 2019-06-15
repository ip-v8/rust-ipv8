pub mod keytypes;
pub mod signature;

use rust_sodium;
use rust_sodium::crypto::sign::ed25519;
use std::error::Error;

create_error!(SignatureError, "Invalid signature");
create_error!(SizeError, "Invalid input size");
create_error!(
    OpenSSLError,
    "OpenSSL had a rapid unscheduled disassembly (oops)"
);

/// wrapper function for signing data using ed25519
pub fn create_signature_ed25519(
    data: &[u8],
    skey: ed25519::SecretKey,
) -> Result<ed25519::Signature, Box<dyn Error>> {
    Ok(ed25519::sign_detached(data, &skey))
}

/// wrapper function for verifying data using ed25519
pub fn verify_signature_ed25519(
    signature: Vec<u8>,
    data: &[u8],
    pkey: ed25519::PublicKey,
) -> Result<bool, Box<dyn Error>> {
    let verify = ed25519::verify_detached(
        &ed25519::Signature::from_slice(&*signature).ok_or_else(|| Box::new(SignatureError))?,
        data,
        &pkey,
    );

    Ok(verify)
}

#[cfg(test)]
mod tests {
    use crate::crypto::verify_signature_ed25519;
    use rust_sodium::crypto::sign::ed25519;

    #[test]
    fn ed25519_verify_signature_error() {
        let seed = ed25519::Seed::from_slice(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();
        let (pkey, _) = ed25519::keypair_from_seed(&seed);

        match verify_signature_ed25519(vec![42], &vec![42], pkey) {
            Ok(_) => assert!(
                false,
                "This shouldn't happen as the signature is malformed thus we expect an error"
            ),
            Err(_) => assert!(true),
        }
    }
}
