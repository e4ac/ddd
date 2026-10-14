[![CICD](https://github.com/e4ac/ddd/actions/workflows/cicd.yml/badge.svg)](https://github.com/e4ac/ddd/actions/workflows/cicd.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit)

# DDD Library
A [Rust](https://rust-lang.org/) library for [Domain-Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design).

## Setup
Include the library in `Cargo.toml`.
```toml
[dependencies]
ddd = { git = "https://github.com/e4ac/ddd" }
```

By default, the library compiles using `std` without `serde` support. You can enable `serde`:
```toml
[dependencies]
ddd = { features = ["serde"], git = "https://github.com/e4ac/ddd" }
```

## TODO
- [ ] Add more features

## Testing
```sh
cargo test --all-targets --all-features
```
