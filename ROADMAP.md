# Provncloud SDK Roadmap

This document tracks planned improvements and future development priorities.

---

## 📈 SDK Expansion Roadmap (Post-Seed)

Multi-language SDK support to improve adoption across different developer ecosystems.

| Language | Priority | Status | Why |
|----------|----------|--------|-----|
| **TypeScript/Node.js** | HIGH | 🔲 Planned | Most web developers use this |
| **Python** | HIGH | 🔲 Planned | Most AI/ML developers use this |
| **Go** | MEDIUM | 🔲 Planned | Backend infra teams |
| **Java** | LOW | 🔲 Future | Enterprise (later) |

**Note:** The REST API is language-agnostic—any language can POST JSON. These SDKs will add convenience wrappers and client-side signing capabilities.

---

## 🔧 Technical Improvements

### HIGH Priority

- [ ] **Integration Tests**
  - Add end-to-end tests: Generate key → Sign claim → Serialize → Deserialize → Verify
  - Simulate full flow with mock API responses

### MEDIUM Priority

- [ ] **Publish to crates.io**
  - Currently installed via `git = "..."` 
  - Publishing increases discoverability
  - Aligns with Rust ecosystem expectations

### LOW Priority

- [ ] **Add `getrandom` for no-std RNG**
  - Currently `generate_keypair()` requires `std` for `OsRng`
  - For true embedded/WASM support, add `getrandom` feature for entropy sources

- [ ] **Add `derive(Eq)` to `Claim`**
  - `Eq` is derivable since all fields implement it
  - Expected when `PartialEq` is present
  ```rust
  #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
  pub struct Claim { ... }
  ```

- [ ] **Version Pinning in Cargo.toml**
  - Dependencies use `version = "1.0"` style
  - Consider pinning to exact minor versions (e.g., `1.0.140`) for reproducible builds

---

## 🎯 Milestones

### v0.2.0 
- [ ] TypeScript/Node.js SDK
- [ ] Python SDK
- [ ] Integration tests
- [ ] Publish to crates.io

### v0.3.0 
- [ ] Go SDK
- [ ] WASM bindings for browser
- [ ] Java SDK (enterprise)

---

## 📝 Notes

- Current Rust SDK is production-ready (v0.1.x)
- REST API works with any language (no SDK required)
- SDK expansion prioritized by developer ecosystem size
