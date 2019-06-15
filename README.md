# rust-ipv8

**Stable:**

[![Build Status](https://travis-ci.org/ip-v8/rust-ipv8.svg?branch=master)](https://travis-ci.org/ip-v8/rust-ipv8)
[![codecov](https://codecov.io/gh/ip-v8/rust-ipv8/branch/master/graph/badge.svg)](https://codecov.io/gh/ip-v8/rust-ipv8)

**Development:**

[![Documentation](https://img.shields.io/badge/rustdoc-ipv8-9E978E.svg)](https://ip-v8.github.io/rust-ipv8/ipv8)
[![Build Status](https://travis-ci.org/ip-v8/rust-ipv8.svg?branch=develop)](https://travis-ci.org/ip-v8/rust-ipv8)
[![codecov](https://codecov.io/gh/ip-v8/rust-ipv8/branch/develop/graph/badge.svg)](https://codecov.io/gh/ip-v8/rust-ipv8)

This is an implementation of the Python library [py-ipv8](https://github.com/Tribler/py-ipv8) in Rust.
The goal is to be completely compatible with its Python counterpart.
This will achieved in a number of phases.

Higher level code (such as communities) will stay in Python. Only the most performance demanding code will be ported to Rust which includes but is not limited to networking and cryptography. Afterwards the tunnel community will follow.

## Differences with py-ipv8

- **Cross-platform**: Given that Rust is a compiled language, _rust-ipv8_ can be build for a lot of platforms. Including native, web (WASM), android (NDK) and even embedded devices.
- **Performance**: Rust is by nature a lot faster than Python, as Python is an interpreted language.
- **Standalone**: This module emits a single binary, which makes distribution easier. Note: Long term
- **Modular**: This module has an arguably better way of structuring the code, making it easier to read, and better testable.

## compilation

To compile rust-ipv8, `cargo` can be used as follows:
```
cargo build
```
and to test use `cargo` like so:
```
cargo test
```

As rust-ipv8 is a library it can alternatively be included in another program's Cargo.toml.
