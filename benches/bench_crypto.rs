#[macro_use]
extern crate criterion;

use criterion::Criterion;
use ipv8::networking::crypto::signature::Signature;
use ipv8::networking::crypto::keytypes::{PrivateKey, PublicKey};
use rust_sodium::crypto::sign::ed25519;

fn e25519_benchmark(c: &mut Criterion) {
  c.bench_function("bench: ed25519", |b| b.iter(|| {
    let seed = ed25519::Seed::from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,]).unwrap();
    let (pkey,skey) = ed25519::keypair_from_seed(&seed);
    let sig = Signature::from_bytes(&[42,43,44],PrivateKey::Ed25519(skey)).unwrap();
    assert_eq!(
      vec![31, 14, 50, 234, 129, 186, 124, 84, 223, 67, 233, 173, 116, 95, 218, 136, 149, 223, 171, 234, 13, 173, 164, 78, 74, 59, 106, 31, 252, 230, 79, 207, 199, 207, 134, 92, 252, 211, 142, 172, 183, 61, 17, 236, 208, 124, 206, 37, 204, 85, 62, 155, 171, 129, 153, 90, 3, 148, 202, 220, 53, 159, 172, 7],
      sig.signature
    );

    assert!(sig.verify(&[42,43,44], PublicKey::Ed25519(pkey)));
  }
  ));
}

fn openssl_verylow_benchmark(c: &mut Criterion) {
  c.bench_function("bench: openssl very low", |b| b.iter(|| {
    // private key generated with SECT163K1 and is always constant because it is directly pasted here
    let skey = openssl::ec::EcKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMFMCAQEEFQKu4aaDxyTSj92iquQP5CIdbagLP6AHBgUrgQQAAaEuAywABABQ76xopUysBuWInGkX+S4elFdpOQZphgLlc6ksoim+5DgUZEBPp+B2Dg==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    //let pkey = openssl::ec::EcKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMEAwEAYHKoZIzj0CAQYFK4EEAAEDLAAEAFDvrGilTKwG5YicaRf5Lh6UV2k5BmmGAuVzqSyiKb7kOBRkQE+n4HYO\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey_tmp = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMEAwEAYHKoZIzj0CAQYFK4EEAAEDLAAEAFDvrGilTKwG5YicaRf5Lh6UV2k5BmmGAuVzqSyiKb7kOBRkQE+n4HYO\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey = pkey_tmp.ec_key().unwrap();


    let sig = Signature::from_bytes(&[42, 43, 44], PrivateKey::OpenSSLVeryLow(skey)).unwrap();
    assert!(sig.verify(&[42, 43, 44], PublicKey::OpenSSLVeryLow(pkey)));
  }
  ));
}
fn openssl_low_benchmark(c: &mut Criterion) {
  c.bench_function("bench: openssl low", |b| b.iter(|| {
    // private key generated with SECT233K1 and is always constant because it is directly pasted here
    let skey = openssl::ec::EcKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMG0CAQEEHQ7vns0bhePCngPc4WeP3wnglzSrml0HdQ+jcpfAoAcGBSuBBAAaoUAD\nPgAEAe2ikH75P/vkdl1Bu8tP/WjOeB6LRxW11qGQNUmUAaFxQ7zff5eZyppMv7D0\n9sRcEuSNjk5nUQgTe6zV\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey_tmp = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMFIwEAYHKoZIzj0CAQYFK4EEABoDPgAEAe2ikH75P/vkdl1Bu8tP/WjOeB6LRxW11qGQNUmUAaFxQ7zff5eZyppMv7D09sRcEuSNjk5nUQgTe6zV\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey = pkey_tmp.ec_key().unwrap();


    let sig = Signature::from_bytes(&[42, 43, 44], PrivateKey::OpenSSLLow(skey)).unwrap();
    // println!("{:?}",sig);

    assert!(sig.verify(&[42, 43, 44], PublicKey::OpenSSLLow(pkey)));
  }));
}

fn openssl_medium_benchmark(c: &mut Criterion) {
  c.bench_function("bench: openssl medium", |b| b.iter(|| {
    // private key generated with SECT409K1 and is always constant because it is directly pasted here
    let skey = openssl::ec::EcKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMIGvAgEBBDNDkh1KSwaBgRj5GGcbYm2qWI5TyBVkOeMVkWWX5+8Dmd44OoSzmR5xCmc1DWuEsasIhhagBwYFK4EEACShbANqAAQAP5r6iYsyTkM7Hea2/tc95iGXV3oCXMLxSWiR/vF/zKjHkPClBN8BQBbBCMjpeS1xLZMUAUi2RoJN69jQevTG+vfhzBNqxIE0dazxbLMvx3wZ6Bol918H8oAa31axHKVaz3SbKLbDTw==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey_tmp = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMH4wEAYHKoZIzj0CAQYFK4EEACQDagAEAD+a+omLMk5DOx3mtv7XPeYhl1d6AlzC\n8Ulokf7xf8yox5DwpQTfAUAWwQjI6XktcS2TFAFItkaCTevY0Hr0xvr34cwTasSB\nNHWs8WyzL8d8GegaJfdfB/KAGt9WsRylWs90myi2w08=\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey = pkey_tmp.ec_key().unwrap();


    let sig = Signature::from_bytes(&[42, 43, 44], PrivateKey::OpenSSLMedium(skey)).unwrap();
    // println!("{:?}",sig);

    assert!(sig.verify(&[42, 43, 44], PublicKey::OpenSSLMedium(pkey)));
  }));
}

fn openssl_high_benchmark(c: &mut Criterion) {
  c.bench_function("bench: openssl high", |b| b.iter(|| {
    // private key generated with SECT571R1 and is always constant because it is directly pasted here
    let skey = openssl::ec::EcKey::private_key_from_pem("-----BEGIN EC PRIVATE KEY-----\nMIHuAgEBBEgCQPcwiTfJz3T0/fDqAgvtTO3fvCobbxvJAnsDKQwjJbK9Ak2njemFanI8BOGp/1Mi6nrjfJs9+8h9LhUIYsrJ2j7piRxo2SygBwYFK4EEACehgZUDgZIABAJW+0vOn4V4P7Drsg4IxTtrM7OLA5sUwnBxDyhDcyXfmAdmmtZabrTiBb5jozZ0rXkoUIGOUnaaYH+k+NlbDVBbXtIQbmwpOQTzMTTC/oJi5TJUFc6G3529hTLStV3lILPks4SPk2DPRDC4oC/jRpMXn9VphjzT4gjruhTxVaoEAyi3YmdQpIBXzWVD/lOOhQ==\n-----END EC PRIVATE KEY-----".as_bytes()).unwrap();
    let pkey_tmp = openssl::pkey::PKey::public_key_from_pem("-----BEGIN PUBLIC KEY-----\nMIGnMBAGByqGSM49AgEGBSuBBAAnA4GSAAQCVvtLzp+FeD+w67IOCMU7azOziwObFMJwcQ8oQ3Ml35gHZprWWm604gW+Y6M2dK15KFCBjlJ2mmB/pPjZWw1QW17SEG5sKTkE8zE0wv6CYuUyVBXOht+dvYUy0rVd5SCz5LOEj5Ngz0QwuKAv40aTF5/VaYY80+II67oU8VWqBAMot2JnUKSAV81lQ/5TjoU=\n-----END PUBLIC KEY-----".as_bytes()).unwrap();
    let pkey = pkey_tmp.ec_key().unwrap();


    let sig = Signature::from_bytes(&[42, 43, 44], PrivateKey::OpenSSLHigh(skey)).unwrap();
    // println!("{:?}",sig);

    assert!(sig.verify(&[42, 43, 44], PublicKey::OpenSSLHigh(pkey)));
  }));
}

criterion_group!(benches, e25519_benchmark, openssl_verylow_benchmark, openssl_low_benchmark, openssl_medium_benchmark, openssl_high_benchmark);
criterion_main!(benches);
