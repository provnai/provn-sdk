# Provncloud SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/crates/v/provn-sdk?label=Rust%20%28crates.io%29&color=orange)](https://crates.io/crates/provn-sdk)
[![NPM](https://img.shields.io/npm/v/%40provncloud%2Fsdk?label=NPM&color=red)](https://www.npmjs.com/package/@provncloud/sdk)
[![PyPI](https://img.shields.io/pypi/v/provn-sdk?label=PyPI&color=yellow)](https://pypi.org/project/provn-sdk/)
[![Go](https://img.shields.io/github/v/tag/provnai/provn-sdk?label=Go&color=blue)](https://pkg.go.dev/github.com/provnai/provn-sdk/go)
[![GitHub Release](https://img.shields.io/github/v/release/provnai/provn-sdk?label=GitHub%20Release&color=black)](https://github.com/provnai/provn-sdk/releases)
[![GitHub Tags](https://img.shields.io/github/v/tag/provnai/provn-sdk?label=Tags&color=gray)](https://github.com/provnai/provn-sdk/tags)

Provncloud SDK is a multi-language toolkit for deterministic claim serialization, Ed25519 signing, SHA-256 hashing, and offline verification.

The repository is built around a shared wire format:

- `Claim { data, metadata?, timestamp }`
- `SignedClaim { claim, public_key, signature }`

The Rust crate is the reference implementation. TypeScript uses Rust via WASM, Python uses Rust via PyO3, and Go and Java are native ports that match the same canonical JSON and signature behavior.

## Quickstart

### Python
```bash
pip install provn-sdk
```

```python
from provn_sdk import ProvnSDK
import time

sdk = ProvnSDK()
keys = sdk.generate_keypair()
claim = sdk.create_claim("My Critical Data", int(time.time()), None)
signed = sdk.sign_claim(claim, keys["private_key"])

print(sdk.verify_claim(signed))
```

### TypeScript / Node.js
```bash
npm install @provncloud/sdk
```

```ts
import { ProvnSDK, initSDK } from "@provncloud/sdk";

async function main() {
  await initSDK();

  const sdk = new ProvnSDK();
  const keys = sdk.generateKeypair();
  sdk.setKeypair(keys);

  const claim = sdk.createClaimNow("My Critical Data");
  const signed = sdk.signClaim(claim);

  console.log(sdk.verifyClaim(signed));
}

main();
```

### Rust
```bash
cargo add provn-sdk
```

```rust
use provn_sdk::{generate_keypair, sign_claim, verify_claim, Claim};

fn main() {
    let key = generate_keypair();
    let claim = Claim::new("My Critical Data".to_string(), None);
    let signed = sign_claim(&claim, &key).unwrap();

    println!("{}", verify_claim(&signed).unwrap());
}
```

### Go
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
	keypair, _ := provnsdk.GenerateKeypair()
	claim := provnsdk.CreateClaimWithoutMetadata("My Critical Data")
	signed, _ := provnsdk.SignClaim(claim, keypair)
	isValid, _ := provnsdk.VerifyClaim(signed)

	fmt.Println(isValid)
}
```

## Security Notes

- The SDK enforces a 2 KB limit on the canonical JSON payload that gets signed.
- For large or sensitive documents, hash locally and sign the hash instead of raw content.
- When parsing untrusted JSON, prefer strict deserialization paths such as Go's `ParseSignedClaimStrict`.
- Python raises `MalformedClaimError` for malformed verification input and returns `False` only for clean cryptographic mismatch.

## OSS Hygiene

- The committed keys in `spec/test-vectors.json` are public test-only vectors and provide zero production security.
- Built native extensions, wheels, `.wasm` outputs, `dist/`, `target/`, and cross-language scratch artifacts are intentionally excluded from source control.
- No production API keys, secrets, or private credentials should ever be committed to this repository.

## Development

```bash
python scripts/build-all.py build
python scripts/build-all.py test
python scripts/build-all.py clean
```

For contributor guidance, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Documentation

- [Protocol Specification](spec/SPEC.md)
- [Use Cases](USE_CASES.md)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

## License

MIT. See [LICENSE](LICENSE).
