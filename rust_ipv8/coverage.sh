#!/usr/bin/env bash
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
cargo build --verbose $CARGO_OPTIONS
cargo test --verbose $CARGO_OPTIONS
zip -0 ccov.zip `find ../ \( -name "*ipv8*.gc*" \) -print`
grcov ccov.zip -s src/ -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" -o lcov.info
rm ccov.zip
cd src
genhtml -o ../../target/coverage/ ../lcov.info
xdg-open ../../target/coverage/index.html
