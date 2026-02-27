# Provncloud SDK (Rust)

[![Crates.io](https://img.shields.io/crates/v/provn-sdk)](https://crates.io/crates/provn-sdk)
[![Docs.rs](https://docs.rs/provn-sdk/badge.svg)](https://docs.rs/provn-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

**Universal, no-std Rust SDK for privacy-preserving digital signatures and data anchoring.**

Provncloud SDK allows you to cryptographically sign data and anchor it to blockchain networks (like Arweave AO and Solana) without revealing the raw content. It strictly adheres to [JCS (RFC 8785)](https://rfc-editor.org/rfc/rfc8785) for canonical JSON serialization, ensuring signatures verified in Rust can be validated across any other supported language.

## 🚀 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
provn-sdk = "0.3.0"
```

For `no_std` environments (e.g., Solana programs):

```toml
[dependencies]
provn-sdk = { version = "0.3.0", default-features = false, features = ["alloc"] }
```

## 💻 Usage

```rust
use provn_sdk::{Claim, sign_claim, verify_claim, generate_keypair};

fn main() {
    // 1. Generate a new Ed25519 keypair
    let key = generate_keypair();

    // 2. Create a claim with the data you want to anchor
    let claim = Claim::new("AI Model Accuracy: 99.2%".to_string());

    // 3. Sign the claim
    let signed = sign_claim(&claim, &key).expect("Signing failed");

    // 4. Verify the claim (Offline)
    let is_valid = verify_claim(&signed).expect("Verification failed");
    println!("Signature valid: {}", is_valid);
}
```

## 🛠 Features

- **Standard Compliance**: Uses JCS (RFC 8785) for deterministic JSON serialization.
- **no-std Support**: Fully compatible with embedded and blockchain runtimes.
- **Cross-Language**: Signatures are interoperable with Provn SDKs in Python, JS/TS, Go, and Java.
- **Safety**: Robust validation of payload sizes (2KB limit) and timestamp bounds.

## 📚 Resources

- [**Monorepo & Examples**](https://github.com/provnai/provn-sdk)
- [**Protocol Spec**](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## ⚖️ License

MIT License. See [LICENSE](https://github.com/provnai/provn-sdk/blob/main/LICENSE) for details.
