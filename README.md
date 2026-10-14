[![CICD](https://github.com/e4ac/ddd/actions/workflows/cicd.yml/badge.svg)](https://github.com/e4ac/ddd/actions/workflows/cicd.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit)

# DDD Library
A [Rust](https://rust-lang.org/) library for [Domain-Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design).

## Usage
Include the library in `Cargo.toml`.
```toml
[dependencies]
ddd = { git = "https://github.com/e4ac/ddd" }
```

Available features:
- `chrono`: adds time support using [`chrono`](https://crates.io/crates/chrono) crate
- `serde`: adds serialization and deserialization support using [`serde`](https://crates.io/crates/serde) crate
- `uuid`: adds UUID support using [`uuid`](https://crates.io/crates/uuid) crate

## Testing
```sh
cargo test --all-targets --all-features
```

## TODO
- [ ] Add more features
