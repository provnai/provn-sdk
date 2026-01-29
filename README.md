# Provncloud SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-blue.svg)](https://www.rust-lang.org/)
[![Rust CI](https://github.com/provnai/provn-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/provnai/provn-sdk/actions/workflows/ci.yml)

Provncloud SDK is a lightweight cryptographic toolkit for signing and verifying data claims. It allows you to generate local audit trails—ensuring that sensitive data never leaves your environment while still providing a permanent, verifiable proof anchored to [Arweave AO](https://ao.arweave.dev) and [Solana](https://solana.com).

---

## 🏗️ How it Works: The Anchoring Flow

This SDK is the first step in a decentralized proof-of-existence pipeline:

1.  **Local Signing**: You sign a data hash using your private Ed25519 key (Identity).
2.  **API Submission**: You send the signed claim to the Provncloud API. The API verifies your signature but never sees your raw data.
3.  **L3 Batching**: Provncloud batches multiple claims into an "Industrial Receipt" for high-throughput efficiency.
4.  **Dual Anchoring**:
    - The full audit log is permanently stored on Arweave AO.
    - A cryptographic commitment (state root) is anchored to Solana for high-speed finality and settlement.

---

## 🧭 Core Principles

This SDK is built to give you full control over your data identity:
1.  **Identity Control**: You manage your own Ed25519 keys. Only your signatures are shared, never your private keys.
2.  **Independent Verification**: You can verify any claim receipt locally and independently, with no reliance on central servers.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
provn-sdk = { git = "https://github.com/provnai/provn-sdk.git" }
```

For no-std environments (e.g., Solana programs):
```toml
[dependencies]
provn-sdk = { git = "https://github.com/provnai/provn-sdk.git", default-features = false, features = ["alloc"] }
```

---

## 🚀 Quick Start

### 1. Generate a Sovereign Keypair
```rust
use provn_sdk::generate_keypair; // Requires "std"

// Create a new signing key (Ed25519)
let signing_key = generate_keypair();
let public_key = hex::encode(signing_key.verifying_key().as_bytes());

println!("Digital Identity: ed25519:{}", public_key);
```

### 2. Sign a Local Claim
```rust
use provn_sdk::{Claim, sign_claim, compute_hash};

// For privacy, hash your data first (raw data stays on your device)
let data = "AI Model v1.0 Accuracy: 98.42%";
let hash = compute_hash(data.as_bytes());

// Create a claim with the hash
let claim = Claim::new(hash); // Uses current system time (Requires "std")

// Sign locally (Data remains on your device)
let signed_claim = sign_claim(&claim, &signing_key).expect("Signing failed");

println!("Claim Hash: {}", signed_claim.claim.data);
println!("Signature: {}", signed_claim.signature);
```

### 3. Verify a Receipt
```rust
use provn_sdk::verify_claim;

// Verify the integrity of a signed claim
let is_valid = verify_claim(&signed_claim).unwrap_or(false);
assert!(is_valid);
```

---

## 🛠️ Technical Architecture

This library is a portable implementation of the `ed25519-dalek` crate, designed to provide a consistent identity layer across different environments—from web servers to resource-constrained on-chain processes.

| Feature | Implementation | Complexity |
| :--- | :--- | :--- |
| **Cryptography** | Ed25519 (Edwards-curve Digital Signature Algorithm) | O(1) Verification |
| **Serialization** | JCS (RFC 8785) Canonical JSON | Sorted Keys |
| **Runtime** | `no-std` + `alloc` | Solana/AO Compatible |
| **Payload Capacity** | 2KB (Optimized for L3 Batching) | High throughput |

---

## ⚖️ License

Distributed under the MIT License. See `LICENSE` for more information.

---

## 🔗 Ecosystem

- [Provncloud](https://provncloud.com/)
- [Arweave AO](https://ao.arweave.dev)
- [Solana](https://solana.com)
