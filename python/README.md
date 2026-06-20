# Provncloud SDK (Python)

[![PyPI](https://img.shields.io/pypi/v/provn-sdk)](https://pypi.org/project/provn-sdk/)
[![Python Version](https://img.shields.io/pypi/pyversions/provn-sdk)](https://pypi.org/project/provn-sdk/)
[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

Python bindings for the Provncloud signing format, powered by the Rust core via PyO3.

## Installation

```bash
pip install provn-sdk
```

## Usage

```python
import time
from provn_sdk import MalformedClaimError, ProvnSDK

sdk = ProvnSDK()
keys = sdk.generate_keypair()
claim = sdk.create_claim("AI Model Metadata v2.4", int(time.time()), None)
signed = sdk.sign_claim(claim, keys["private_key"])

try:
    print(sdk.verify_claim(signed))
except MalformedClaimError as exc:
    print(f"bad input: {exc}")
```

## Verification Behavior

- Returns `True` for a valid signature.
- Returns `False` for a clean cryptographic mismatch.
- Raises `MalformedClaimError` for malformed verification input.

## Resources

- [Monorepo](https://github.com/provnai/provn-sdk)
- [Protocol Spec](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## License

MIT
