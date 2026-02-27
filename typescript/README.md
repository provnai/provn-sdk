# Provncloud SDK (TypeScript/Node.js)

[![NPM](https://img.shields.io/npm/v/@provncloud/sdk)](https://www.npmjs.com/package/@provncloud/sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

**Official TypeScript/JavaScript SDK for privacy-preserving digital signatures and data anchoring.**

Provncloud SDK allows you to cryptographically sign data and anchor it to blockchain networks like Arweave AO and Solana without revealing the raw content. It uses a high-performance WebAssembly (WASM) core to ensure cryptographic integrity and cross-platform consistency.

## 🚀 Installation

### Using npm:
```bash
npm install @provncloud/sdk
```

### Using yarn:
```bash
yarn add @provncloud/sdk
```

## 💻 Usage

The SDK is designed to worked in both Node.js and Browser environments.

```typescript
import { ProvnSDK, initSDK } from '@provncloud/sdk';

async function main() {
  // 1. Initialize the WASM engine
  await initSDK();
  
  const sdk = new ProvnSDK();

  // 2. Generate a new Ed25519 keypair
  const identity = sdk.generateKeypair();

  // 3. Create and sign a claim
  const claim = sdk.createClaimNow("User Verified Identity #1234");
  const signed = sdk.signClaim(claim);

  // 4. Verify (Offline)
  const isValid = sdk.verifyClaim(signed);
  console.log("Signature valid:", isValid);
}

main();
```

## 🛠 Features

- **High Performance**: Cryptographic core implemented in Rust and compiled to WASM.
- **Protocol Consistency**: Strictly follows [JCS (RFC 8785)](https://rfc-editor.org/rfc/rfc8785) for canonical JSON serialization.
- **Unified API**: Synchronous signing and verifying after WASM initialization.
- **Cross-Platform**: Compatible with Node.js 18+ and all modern browsers.

## 📚 Resources

- [**Monorepo & Examples**](https://github.com/provnai/provn-sdk)
- [**Protocol Spec**](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## ⚖️ License

MIT License. See [LICENSE](https://github.com/provnai/provn-sdk/blob/main/LICENSE) for details.
