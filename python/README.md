# Provncloud SDK (Python)

[![PyPI](https://img.shields.io/pypi/v/provn-sdk)](https://pypi.org/project/provn-sdk/)
[![Python Version](https://img.shields.io/pypi/pyversions/provn-sdk)](https://pypi.org/project/provn-sdk/)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

**Official Python SDK for privacy-preserving digital signatures and data anchoring.**

Provncloud SDK allows you to cryptographically sign data and anchor it to blockchain networks like Arweave AO and Solana without revealing the raw content. Built with a high-performance Rust core (via PyO3), it ensures strict cryptographic standards and cross-platform interoperability.

## 🚀 Installation

```bash
pip install provn-sdk
```

## 💻 Usage

```python
from provn_sdk import ProvnSDK
import time

# 1. Initialize the SDK
sdk = ProvnSDK()

# 2. Generate a new Ed25519 keypair
keys = sdk.generate_keypair()

# 3. Create a claim with current timestamp
timestamp = int(time.time())
claim = sdk.create_claim("AI Model Metadata v2.4", timestamp)

# 4. Sign the claim
signed = sdk.sign_claim(claim, keys['private_key'])

# 5. Verify (Offline)
is_valid = sdk.verify_claim(signed)
print(f"Signature valid: {is_valid}")
```

## 🛠 Features

- **Rust-Powered**: High-performance cryptographic operations using native bindings.
- **Deterministic**: Implements JCS (RFC 8785) for cross-language signature compatibility.
- **Payload Safety**: Enforces 2KB limits on claim data to prevent broadcast bloat.
- **Easy Integration**: Simple object-oriented API for key management and anchoring.

## 📚 Resources

- [**Monorepo & Examples**](https://github.com/provnai/provn-sdk)
- [**Protocol Spec**](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## ⚖️ License

MIT License. See [LICENSE](https://github.com/provnai/provn-sdk/blob/main/LICENSE) for details.
