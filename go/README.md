# Provncloud SDK (Go)

[![Go Reference](https://pkg.go.dev/badge/github.com/provnai/provn-sdk/go.svg)](https://pkg.go.dev/github.com/provnai/provn-sdk/go)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

**Official Go SDK for privacy-preserving digital signatures and data anchoring.**

Provncloud SDK allows you to cryptographically sign data and anchor it to blockchain networks like Arweave AO and Solana without revealing the raw content.

## 🚀 Installation

```bash
go get github.com/provnai/provn-sdk/go
```

## 💻 Usage

```go
package main

import (
	"fmt"
	provnsdk "github.com/provnai/provn-sdk/go/pkg"
)

func main() {
	// 1. Generate a new Ed25519 keypair
	keypair, _ := provnsdk.GenerateKeypair()

	// 2. Create a claim
	claim := provnsdk.CreateClaim("IoT Sensor Data: 22.5C", "{\"sensor_id\": \"S1\"}")

	// 3. Sign the claim
	signed, _ := provnsdk.SignClaim(claim, keypair)

	// 4. Verify (Offline)
	isValid, _ := provnsdk.VerifyClaim(signed)
	fmt.Printf("Signature valid: %v\n", isValid)
}
```

## 🛠 Features

- **Pure Go**: No CGO dependencies for the core logic.
- **Protocol Compliance**: Strictly follows JCS (RFC 8785) for canonical JSON serialization using Go's native capabilities.
- **Interoperable**: Signatures are compatible with Provn SDKs in Rust, Python, TS, and Java.

## 📚 Resources

- [**Monorepo & Examples**](https://github.com/provnai/provn-sdk)
- [**Protocol Spec**](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## ⚖️ License

MIT License. See [LICENSE](https://github.com/provnai/provn-sdk/blob/main/LICENSE) for details.
