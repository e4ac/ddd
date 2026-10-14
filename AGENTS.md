# DDD
A Rust library for Domain-Driven Design.

## Mandatory Quality Gates
- Ensure no warnings and errors for building by running `cargo build --all-features --all-targets`
- Ensure all unit tests pass by running `cargo test --all-features --all-targets`
- Ensure no warnings and errors for formatting by running `cargo fmt --all -- --check`
- Ensure no warnings and errors for clippy by running by running `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- Must resolve every error and warning before continuing

## Unit Tests
- All unit tests must explicitly follow the **arrange-act-assert (AAA)** pattern
- Ensure coverage handles edge cases (such as null inputs, boundary limits) when generating tests
- Rust unit test function names should start with `test_`
