# Publishing Guide

This guide details the steps to publish version `0.2.0` of the Provncloud SDKs.

## Prerequisites
- [ ] `cargo login` (for crates.io)
- [ ] `npm login` (for npmjs.org)
- [ ] `twine` installed (for PyPI)
- [ ] GPG key (for Maven Central)

## 1. Rust SDK (Core)
The core logic resides here. Publish this first.

```bash
cd rust
cargo publish
```

## 2. Rust WASM (Bindings)
Required by the TypeScript SDK.

```bash
cd rust-wasm
wasm-pack build --target nodejs --scope provn
# wasm-pack publish # Optional, usually bundled into JS SDK
```

## 3. TypeScript SDK
Bundles the WASM binary.

```bash
cd typescript
npm run build
npm publish --access public
```

## 4. Python SDK
Native Rust extension.

```bash
cd python
maturin build --release
twine upload target/wheels/*
```

## 5. Go SDK
Go modules are published by tagging the repository.

```bash
git tag v0.2.0
git push origin v0.2.0
# Users can now: go get github.com/provnai/provn-sdk/go@v0.2.0
```

## 6. Java SDK
Deploy to Maven Central (requires complex setup).

```bash
cd java
mvn clean deploy
```
