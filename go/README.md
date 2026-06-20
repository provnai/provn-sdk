# Provncloud SDK (Go)

[![Go Reference](https://pkg.go.dev/badge/github.com/provnai/provn-sdk/go.svg)](https://pkg.go.dev/github.com/provnai/provn-sdk/go)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

Native Go implementation of the Provncloud claim signing format.

## Installation

```bash
go get github.com/provnai/provn-sdk/go
```

## Usage

```go
package main

import (
	"fmt"

	provnsdk "github.com/provnai/provn-sdk/go/pkg"
)

func main() {
	keypair, _ := provnsdk.GenerateKeypair()
	claim := provnsdk.CreateClaimWithoutMetadata("IoT Sensor Data: 22.5C")
	signed, _ := provnsdk.SignClaim(claim, keypair)
	isValid, _ := provnsdk.VerifyClaim(signed)

	fmt.Println(isValid)
}
```

## Metadata Semantics

- `CreateClaim(data, metadata)` preserves the metadata string exactly, including `""`.
- `CreateClaimWithoutMetadata(data)` omits the metadata field entirely.
- `CreateClaimWithTimestamp(data, timestamp, metadataPtr)` gives full control over omission vs. explicit empty string.

## Security Notes

- `ParseSignedClaimStrict` should be used when deserializing untrusted JSON.
- The canonical JSON payload is limited to 2 KB.

## Resources

- [Monorepo](https://github.com/provnai/provn-sdk)
- [Protocol Spec](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## License

MIT
