#[macro_use]
extern crate criterion;

use criterion::{Criterion, black_box};
use rust_ipv8::crypto::signature::{KeyPair, sign, verify};
use untrusted::Input;

fn e25519_benchmark(c: &mut Criterion) {
    c.bench_function("bench: ed25519", |b| {
        let pk1 = KeyPair::from_seed_unchecked(&[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ])
        .unwrap();

        let public = pk1.public_key().unwrap();

        b.iter(|| {
            let sig = sign(&pk1, &[42, 42, 42]).unwrap();
            let res = verify(
                Input::from(&public),
                Input::from(&[42, 42, 42]),
                Input::from(&sig.0),
            );
            black_box(res);
        })
    });
}

criterion_group!(benches, e25519_benchmark);
criterion_main!(benches);
