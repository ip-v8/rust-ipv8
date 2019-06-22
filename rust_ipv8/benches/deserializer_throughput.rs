use criterion::*;
use rust_ipv8::serialization::Packet;
use rust_ipv8::serialization::header::Header;
use rust_ipv8::payloads::timedistributionpayload::TimeDistributionPayload;
use rust_ipv8::payloads::introductionresponsepayload::IntroductionResponsePayload;
use rust_ipv8::payloads::binmemberauthenticationpayload::BinMemberAuthenticationPayload;

/// Critirion benchmark example
/// ```
/// fn my_bench(c: &mut Criterion) {
///    // One-time setup code goes here
///    c.bench_function("my_bench", |b| {
///        // Per-sample (note that a sample can be many iterations) setup goes here
///        b.iter(|| {
///            // Measured code goes here
///        });
///    });
///}

fn throughput(c: &mut Criterion) {
    // These are the bytes of packet number 1
    static BYTES: [u8; 208] = [
        0x00, 0x02, 0xba, 0xf3, 0x0e, 0xd9, 0x19, 0x2b, 0xa3, 0x54, 0xcd, 0xd7, 0xb1, 0x73, 0xe0,
        0xef, 0x2c, 0x32, 0x80, 0x27, 0xf1, 0xd3, 0xf5, 0x00, 0x4a, 0x4c, 0x69, 0x62, 0x4e, 0x61,
        0x43, 0x4c, 0x50, 0x4b, 0x3a, 0x51, 0xe7, 0x12, 0xc4, 0xeb, 0x8a, 0xc2, 0x5a, 0xe3, 0xa5,
        0x68, 0x24, 0x08, 0xb2, 0xad, 0xbd, 0x6b, 0x78, 0xa4, 0x25, 0x54, 0x7f, 0x26, 0x85, 0xcf,
        0xdf, 0x1e, 0xe9, 0x27, 0x0c, 0xbe, 0x7e, 0xc3, 0x36, 0xc4, 0x16, 0x0f, 0xf5, 0x72, 0x05,
        0x4c, 0x87, 0x78, 0x42, 0xbe, 0x37, 0x73, 0x50, 0x45, 0xa9, 0x3b, 0xc4, 0xe2, 0x04, 0x15,
        0x31, 0x6f, 0xdb, 0x14, 0x71, 0x61, 0xa2, 0xd7, 0x46, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x51, 0xab, 0x1b, 0xc2, 0x2b, 0x67, 0xc0, 0xa8, 0x01, 0x4b, 0x1f, 0x9a, 0xc0,
        0xa8, 0x01, 0x4b, 0x1f, 0x9a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xd2, 0x0e, 0x00, 0x00, 0x00, 0x00, 0xce, 0xe9, 0x32, 0x6b, 0x9d, 0xd4,
        0xbb, 0x8a, 0xaf, 0x8d, 0xc0, 0x39, 0x28, 0x8e, 0xbf, 0xc2, 0x4a, 0x10, 0xad, 0xc3, 0x7a,
        0xf1, 0xd9, 0xc8, 0x04, 0x17, 0x72, 0x5d, 0x2d, 0x3e, 0x5e, 0x07, 0x52, 0x4d, 0xab, 0x6e,
        0xa7, 0x1b, 0x17, 0x5a, 0x77, 0x5d, 0xb5, 0xd8, 0x91, 0x0c, 0x2b, 0x4b, 0xc8, 0xbb, 0x03,
        0xd3, 0x55, 0xed, 0x10, 0x26, 0xdd, 0xbb, 0xd8, 0xb2, 0x3b, 0xfd, 0xfc, 0x01,
    ];

    let packet = Packet(BYTES.to_vec());

    c.bench(
        "throughput",
        Benchmark::new("simple-deserialize", |b| {
            b.iter(|| {
                let data = Packet(BYTES.to_vec());
                let mut de = data.start_deserialize();
                let _: Header = de.pop_header().unwrap();
                // de.verify();
                let _: TimeDistributionPayload = de.next_payload().unwrap();
                let _: IntroductionResponsePayload = de.next_payload().unwrap();
            })
        })
        .throughput(Throughput::Bytes(BYTES.len() as u32)),
    );

    c.bench(
        "throughput",
        Benchmark::new("only-bin-member-auth", |b| {
            b.iter(|| {
                let data = Packet(BYTES.to_vec());
                let de = data.start_deserialize();
                let bin: BinMemberAuthenticationPayload =
                    de.skip_header().unwrap().next_payload().unwrap();
            })
        })
        .throughput(Throughput::Bytes(BYTES.len() as u32)),
    );

    c.bench(
        "throughput",
        Benchmark::new("only-verify", |b| {
            b.iter(|| {
                let data = Packet(BYTES.to_vec());
                let de = data.start_deserialize();
                de.skip_header().unwrap().verify();
            })
        })
        .throughput(Throughput::Bytes(BYTES.len() as u32)),
    );

    c.bench(
        "throughput",
        Benchmark::new("deserialize+verify", |b| {
            b.iter(|| {
                let data = Packet(BYTES.to_vec());
                let mut de = data.start_deserialize();
                let _: Header = de.pop_header().unwrap();
                de.verify();
                let _: TimeDistributionPayload = de.next_payload().unwrap();
                let _: IntroductionResponsePayload = de.next_payload().unwrap();
            })
        })
        .throughput(Throughput::Bytes(BYTES.len() as u32)),
    );
}

criterion_group!(benches, throughput);
criterion_main!(benches);
