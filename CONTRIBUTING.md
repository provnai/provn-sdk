# Contributing to Provncloud SDK

Thank you for your interest in contributing to the Provncloud SDK! We welcome contributions from the community to help make sovereign identity more accessible and robust.

## 🚀 Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/provn-sdk.git
   ```
3. **Install Rust**: Ensure you have the latest stable version of Rust installed.
4. **Run tests**:
   ```bash
   cargo test
   ```


## 🏗️ Monorepo Architecture

This repository contains SDK implementations in multiple languages:

```
provn-sdk/
├── spec/           # Shared specification (single source of truth)
├── rust/           # Core Rust SDK (no-std compatible)
├── rust-wasm/      # WebAssembly bindings
├── typescript/     # TypeScript/Node.js SDK
├── python/         # Python SDK (Native Rust extension via PyO3)
├── go/             # Go SDK
├── java/           # Java SDK
└── examples/       # Usage examples for each language
```

## 🛠️ Developer Management (`manage.py`)

We provide a unified script for building and testing the entire monorepo.

```bash
# Build all SDKs
python manage.py build

# Run all tests
python manage.py test

# Clean artifacts
python manage.py clean
```

## 🧪 Testing

We use standard `cargo test`. Please ensure all tests pass before submitting a Pull Request.

```bash
# Test with standard library
cargo test

# Test no-std compatibility
cargo check --no-default-features --features alloc
```

## 📮 Pull Request Process

1. Create a new branch for your feature or bugfix.
2. Commit your changes with descriptive messages.
3. Push to your fork and submit a Pull Request to the `main` branch.
4. Ensure the CI pipeline passes.

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the **MIT License**.

