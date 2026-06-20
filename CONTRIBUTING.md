# Contributing to Provncloud SDK

Thanks for contributing.

## Repository Layout

```text
provn-sdk/
├── spec/         # Shared protocol spec and test vectors
├── rust/         # Reference implementation
├── rust-wasm/    # WASM bindings used by TypeScript
├── typescript/   # TypeScript SDK
├── python/       # Python SDK via PyO3
├── go/           # Go SDK
├── java/         # Java SDK
├── examples/     # Language examples
└── scripts/      # Monorepo tooling
```

## Local Development

Use the shared helper script from the repo root:

```bash
python scripts/build-all.py build
python scripts/build-all.py test
python scripts/build-all.py clean
```

Language-specific checks:

```bash
# Rust
cd rust
cargo test --all-features
cargo check --no-default-features --features alloc

# TypeScript
cd typescript
npm ci
npm run build
npm test -- --runInBand

# Python
cd python
python -m pip install maturin pytest
maturin develop --release
pytest -v

# Go
cd go
go test -v -race ./...
go vet ./...

# Java
cd java
mvn clean test
```

## Cross-Language Checks

- Shared interoperability vectors live in `spec/test-vectors.json`.
- If you change canonical serialization, signing, or verification behavior, update the relevant vectors and tests together.
- Treat every committed key in the shared vectors as public test-only material.

## OSS Hygiene

- Never commit production secrets, API keys, customer data, or private credentials.
- Never commit built artifacts such as `.so`, `.pyd`, `.wasm`, `dist/`, `target/`, or cross-language scratch outputs.
- Keep examples and docs free of proprietary or internal-only references.

## Pull Requests

1. Create a focused branch.
2. Update code, tests, and public docs together.
3. Make sure the relevant local checks pass.
4. Open a PR against `main`.

## License

By contributing, you agree that your contributions are licensed under the MIT License.
