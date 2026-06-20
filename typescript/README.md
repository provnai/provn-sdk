# Provncloud SDK (TypeScript / Node.js)

[![NPM](https://img.shields.io/npm/v/@provncloud/sdk)](https://www.npmjs.com/package/@provncloud/sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

TypeScript SDK backed by the Rust reference implementation compiled to WebAssembly.

## Installation

```bash
npm install @provncloud/sdk
```

## Usage

```ts
import { ProvnSDK, initSDK } from "@provncloud/sdk";

async function main() {
  await initSDK();

  const sdk = new ProvnSDK();
  const keys = sdk.generateKeypair();
  sdk.setKeypair(keys);

  const claim = sdk.createClaimNow("User Verified Identity #1234");
  const signed = sdk.signClaim(claim);

  console.log(sdk.verifyClaim(signed));
}

main();
```

## Verification Behavior

- Returns `true` for a valid signature.
- Returns `false` for cryptographic failure.
- Throws `VerificationError` for malformed input.

## Resources

- [Monorepo](https://github.com/provnai/provn-sdk)
- [Protocol Spec](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## License

MIT
