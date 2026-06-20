# Provncloud SDK (Rust)

[![Crates.io](https://img.shields.io/crates/v/provn-sdk)](https://crates.io/crates/provn-sdk)
[![Docs.rs](https://docs.rs/provn-sdk/badge.svg)](https://docs.rs/provn-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

Reference Rust implementation for deterministic claim serialization, Ed25519 signing, SHA-256 hashing, and offline verification.

## Installation

```toml
[dependencies]
provn-sdk = "0.3.3"
```

For `no_std` environments:

```toml
[dependencies]
provn-sdk = { version = "0.3.3", default-features = false, features = ["alloc"] }
```

## Usage

```rust
use provn_sdk::{generate_keypair, sign_claim, verify_claim, Claim};

fn main() {
    let key = generate_keypair();
    let claim = Claim::new("AI Model Accuracy: 99.2%".to_string(), None);
    let signed = sign_claim(&claim, &key).expect("signing failed");

    println!("{}", verify_claim(&signed).expect("verification failed"));
}
```

## Notes

- Payloads are capped at 2 KB after canonical JSON serialization.
- `Claim::new_with_timestamp` validates both timestamp bounds and empty data.
- This crate is the behavioral reference for the other SDKs in the monorepo.

## Resources

- [Monorepo](https://github.com/provnai/provn-sdk)
- [Protocol Spec](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## License

MIT
