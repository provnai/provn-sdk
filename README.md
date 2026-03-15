# Provncloud SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/crates/v/provn-sdk?label=Rust%20%28crates.io%29&color=orange)](https://crates.io/crates/provn-sdk)
[![NPM](https://img.shields.io/npm/v/%40provncloud%2Fsdk?label=NPM&color=red)](https://www.npmjs.com/package/@provncloud/sdk)
[![PyPI](https://img.shields.io/pypi/v/provn-sdk?label=PyPI&color=yellow)](https://pypi.org/project/provn-sdk/)
[![Go](https://img.shields.io/github/v/tag/provnai/provn-sdk?label=Go&color=blue)](https://pkg.go.dev/github.com/provnai/provn-sdk/go)

**The standard for privacy-preserving digital signatures and data anchoring.**
Provncloud SDK allows you to cryptographically sign data and anchor it to Solana without revealing the raw content.

---

## 🚀 Quickstart

Choose your language to get started in seconds.

### 🐍 Python
```bash
pip install provn-sdk
```
```python
import provn_sdk
import time

# 1. Generate Identity
keys = provn_sdk.generate_keypair()

# 2. Create & Sign Claim
claim = provn_sdk.create_claim("My Critical Data", int(time.time()), None)
signed = provn_sdk.sign_claim(claim, keys['private_key'])

# 3. Verify (Offline)
is_valid = provn_sdk.verify_claim(signed)
print(f"Verified: {is_valid}")
```

### 📘 TypeScript / Node.js
```bash
npm install @provncloud/sdk
```
```typescript
import { ProvnSDK, init } from '@provncloud/sdk';

async function main() {
  await init(); // Initialize WASM
  const sdk = new ProvnSDK();
  
  // 1. Generate Identity
  const identity = sdk.generateKeypair();
  
  // 2. Sign
  const claim = sdk.createClaimNow("My Critical Data");
  const signed = sdk.signClaim(claim);
  
  // 3. Verify
  const isValid = sdk.verifyClaim(signed);
  console.log("Verified:", isValid);
}
main();
```

### 🦀 Rust
```bash
cargo add provn-sdk
```
```rust
use provn_sdk::{Claim, sign_claim, verify_claim, generate_keypair};

fn main() {
    let key = generate_keypair();
    let claim = Claim::new("My Critical Data".to_string());
    
    let signed = sign_claim(&claim, &key).unwrap();
    let is_valid = verify_claim(&signed).unwrap();
    println!("Verified: {}", is_valid);
}
```

### 🐹 Go
```bash
go get github.com/provnai/provn-sdk/go
```
```go
package main

import (
    "fmt"
    provnsdk "github.com/provnai/provn-sdk/go/pkg"
)

func main() {
    // 1. Generate Identity
    keypair, _ := provnsdk.GenerateKeypair()
    
    // 2. Create & Sign Claim
    claim := provnsdk.CreateClaim("My Critical Data", "")
    signed, _ := provnsdk.SignClaim(claim, keypair)
    
    // 3. Verify (Offline)
    isValid, _ := provnsdk.VerifyClaim(signed)
    fmt.Printf("Verified: %v\n", isValid)
}
```

---

## 📚 Documentation

- [**Protocol Specification**](spec/SPEC.md): Deep dive into the format.
- [**Use Cases**](USE_CASES.md): Real-world application examples.
- [**Contributing**](CONTRIBUTING.md): How to build this monorepo from source.
- [**Changelog**](CHANGELOG.md): Version history and release notes.
- [**Code of Conduct**](CODE_OF_CONDUCT.md): Community guidelines.

## 🔗 Ecosystem
- [Provncloud](https://provncloud.com/)
- [Solana](https://solana.com)

## ⚖️ License
MIT License - see [LICENSE](LICENSE) file
