# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2024-02-08

### Added
- Multi-language SDK support:
  - **Rust** - Core SDK with no-std compatibility (24 tests)
  - **Go** - Pure Go implementation (15 tests)
  - **TypeScript** - Browser/Node.js SDK via WASM
  - **Python** - Python bindings via WASM
  - **Java** - Java SDK with Bouncy Castle
- WebAssembly (WASM) bindings for browser compatibility
- Cross-SDK interoperability testing (Rust ↔ Go verified)
- Comprehensive test vectors in `spec/test-vectors.json`
- Protocol specification (SPEC.md)
- CI/CD workflows for all 5 languages
- Usage examples for all SDKs

### Changed
- Migrated to monorepo structure
- Updated ed25519-dalek from 2.1 to 2.2.0
- Pinned exact dependency versions for reproducible builds
- Added `derive(Eq)` to `Claim` struct
- Added `getrandom` feature for no-std environments

### Security
- All dependencies use latest stable versions
- No known vulnerabilities in dependency tree

## [0.1.0] - 2024-01-29

### Added
- Initial Rust SDK release
- Ed25519 digital signature support
- SHA-256 hashing
- JCS (RFC 8785) canonical JSON serialization
- no-std support for Solana/embedded environments
- 2KB metadata limit enforcement
- Basic claim signing and verification

[Unreleased]: https://github.com/provnai/provn-sdk/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/provnai/provn-sdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/provnai/provn-sdk/releases/tag/v0.1.0
