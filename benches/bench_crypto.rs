#[macro_use]
extern crate criterion;

use criterion::Criterion;
use ipv8::crypto::keytypes::{PrivateKey, PublicKey};
use ipv8::crypto::signature::Signature;
use rust_sodium::crypto::sign::ed25519;

fn create_sign_verify(c: &mut Criterion) {
    c.bench_function("ed25519: creation + sign + verify", |b| {
        b.iter(|| {
            let seed = ed25519::Seed::from_slice(&[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ])
            .unwrap();
            let (pkey, skey) = ed25519::keypair_from_seed(&seed);

            let seed = ed25519::Seed::from_slice(&[
                1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ])
            .unwrap();
            let (e_pkey, e_skey) = ed25519::keypair_from_seed(&seed);

            assert_ne!(e_pkey, pkey);
            assert_ne!(e_skey, skey);

            let sig = Signature::from_bytes(&[42, 43, 44], PrivateKey(e_skey, skey)).unwrap();
            assert_eq!(
                vec![
                    31, 14, 50, 234, 129, 186, 124, 84, 223, 67, 233, 173, 116, 95, 218, 136, 149,
                    223, 171, 234, 13, 173, 164, 78, 74, 59, 106, 31, 252, 230, 79, 207, 199, 207,
                    134, 92, 252, 211, 142, 172, 183, 61, 17, 236, 208, 124, 206, 37, 204, 85, 62,
                    155, 171, 129, 153, 90, 3, 148, 202, 220, 53, 159, 172, 7
                ],
                sig.signature
            );

            assert!(sig.verify(&[42, 43, 44], PublicKey(pkey, pkey)));
        })
    });
}

fn verify(c: &mut Criterion) {
    let seed = ed25519::Seed::from_slice(&[
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ])
    .unwrap();
    let (s_pkey, s_skey) = ed25519::keypair_from_seed(&seed);

    let seed = ed25519::Seed::from_slice(&[
        1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ])
    .unwrap();
    let (e_pkey, e_skey) = ed25519::keypair_from_seed(&seed);

    let sig = Signature::from_bytes(&[42, 43, 44], PrivateKey(e_skey, s_skey)).unwrap();

    // Only bench the actual verification
    c.bench_function("ed25519 verify", move |b| {
        b.iter(|| sig.verify(&[42, 43, 44], PublicKey(s_pkey, e_pkey)))
    });
}

criterion_group!(benches, create_sign_verify, verify);
criterion_main!(benches);
