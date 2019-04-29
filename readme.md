# rust-ipv8

This is an implementation of the Python library [py-ipv8](https://github.com/Tribler/py-ipv8) in Rust. The goal is that it's completely compatible with it's Python counterpart.

## Differences with py-ipv8

- **Cross-platform**: Given that Rust is a compiled language, _rust-ipv8_ can be build for a lot of platforms. Including native, web (WASM), android (NDK) and even embedded devices.
- **Performance**: Rust is by nature a lot faster than Python, as Python is a interpreted language.
- **Standalone**: This module emits a single binary, which makes distribution easier.
- **Modular**: This module has an arguably better way of structuring the code, making it easier to read, and better testable.

## Tests

As this module uses **Cargo** you can run the tests like this:

```
cargo test
```
