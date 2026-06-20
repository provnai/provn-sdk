# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **Python**: `verify_claim` now distinguishes malformed input from cryptographic failure. Malformed payloads raise `MalformedClaimError`; invalid signatures return `False`.
- **Python**: Signing now reuses the validated claim constructor, keeping validation behavior aligned with the Rust core.
- **Go**: `CreateClaim` now preserves explicit empty metadata strings, and `CreateClaimWithoutMetadata` was added for callers who want the field omitted entirely.
- **Go**: Canonical serialization tests now verify `ToSignableBytes()` directly rather than relying on `json.Marshal`.
- **Go**: Shared vector coverage now includes HTML characters, explicit empty metadata, and Unicode edge cases from `spec/test-vectors.json`.
- **TypeScript**: Wrapper tests now exercise the mocked WASM boundary more realistically, including malformed-input vs. invalid-signature behavior.
- **Rust WASM**: Declared the default `console_error_panic_hook` feature cleanly to avoid manifest warnings and preserve better panic diagnostics.

### Changed
- **CI/CD**: `main-ci.yml` now acts as a real top-level changed-paths gate instead of a stub workflow.
- **CI/CD**: Python workflows now use `maturin develop --release` for source-tree testing so the compiled extension is present during test runs.
- **Tooling**: `scripts/build-all.py` now fails fast, reports skipped suites honestly, and uses `npm ci` plus explicit Python tooling installs.

### Documentation
- Refreshed public-facing READMEs and contributing guidance to match the current API surface, versions, and OSS hygiene expectations.
- Clarified that shared test vectors are public test-only material and must never be reused as production credentials.

## [0.3.3] - 2026-05-03

### Security
- **CI/CD**: Removed `|| true` suppressions in TypeScript CI that were masking WASM test failures.
- **Python**: Introduced `MalformedClaimError` to differentiate structural JSON errors from cryptographic failures in `verify_claim`.
- **Java**: Enforced strict immutability on `Claim` and `SignedClaim` models with `public final` fields and `@JsonCreator`.
- **Python/Java**: Synchronized `get_version()` consistency across languages.
- **Go/Java**: Hardened deserializers to reject unknown JSON fields.
- **Hygiene**: Excluded compiled native extensions and cross-language scratch artifacts from source control.

### Fixed
- **TypeScript**: `verifyClaim` now returns `false` on clean cryptographic failure instead of always throwing.

## [0.3.2] - 2026-02-28

### Fixed
- **Python**: Added explicit Python 3.9-3.12 classifiers for PyPI metadata.

## [0.3.1] - 2026-02-28

### Fixed
- **Python**: Resolved missing project metadata in `pyproject.toml`.

## [0.3.0] - 2026-02-27

### Added
- Comprehensive Python test coverage with `pytest`.
- Unified root documentation for the monorepo.
- Automated Python packaging configuration via `pyproject.toml`.

### Changed
- **Breaking**: Unified JSON schema structures to `snake_case` across all languages.
- **Java**: Updated Jackson configuration for cross-language JSON compatibility.
- **TypeScript**: Moved to a synchronous post-initialization signing and verification API.
- **Python**: Fixed the module naming/layout used by Maturin.

### Security
- Reinforced payload validation and timestamp sanity checks.
- Pinned core dependencies more strictly for reproducibility.

## [0.2.0] - 2024-02-08

### Added
- Multi-language SDK support:
  - **Rust**: Core SDK with `no_std` support
  - **Go**: Native Go implementation
  - **TypeScript**: WASM-backed Node.js/browser SDK
  - **Python**: Native Rust extension via PyO3
  - **Java**: Java SDK with Bouncy Castle
- WebAssembly bindings
- Cross-SDK interoperability tests
- Shared test vectors in `spec/test-vectors.json`
- Protocol specification and examples

## [0.1.0] - 2024-01-29

### Added
- Initial Rust SDK release
- Ed25519 digital signatures
- SHA-256 hashing
- Canonical JSON serialization
- `no_std` support
- Payload size enforcement

[Unreleased]: https://github.com/provnai/provn-sdk/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/provnai/provn-sdk/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/provnai/provn-sdk/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/provnai/provn-sdk/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/provnai/provn-sdk/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/provnai/provn-sdk/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/provnai/provn-sdk/releases/tag/v0.1.0
