# Provncloud SDK Specification v1.0

## 1. Overview

This specification defines the Provncloud SDK protocol for privacy-preserving digital signatures and data anchoring.

### Design Goals
- **Privacy**: Raw data never leaves signing device (hash-only mode)
- **Interoperability**: Claims signed in any language can be verified in any other
- **Determinism**: Identical inputs produce identical signatures
- **Efficiency**: Optimized for L3 batching with 2KB metadata limit

## 2. Data Structures

### Claim

```typescript
interface Claim {
  data: string;           // Required. Data hash or raw data
  timestamp: number;      // Required. Unix timestamp (seconds)
  metadata?: string;      // Optional. Additional context
}
```

**Field Ordering**: Alphabetically: `data`, `metadata`, `timestamp`

**Constraints**:
- `data`: Non-empty string, typically SHA-256 hash
- `timestamp`: Positive integer
- `metadata`: If present, serialized claim MUST NOT exceed 2KB

### SignedClaim

```typescript
interface SignedClaim {
  claim: Claim;
  public_key: string;     // Hex-encoded Ed25519 public key (64 hex chars)
  signature: string;      // Hex-encoded Ed25519 signature (128 hex chars)
}
```

## 3. Serialization (JCS - RFC 8785)

All structures MUST be serialized using JCS for deterministic output.

**Example**:
```javascript
// Claim
{ "data": "abc123", "timestamp": 1704067200 }

// Canonical JSON
{"data":"abc123","timestamp":1704067200}
```

**Requirements**:
1. No whitespace
2. Sorted object keys
3. UTF-8 encoding

## 4. Cryptography

### SHA-256 Hashing

```
Function: compute_hash(data: &[u8]) -> String
Input: Arbitrary bytes
Output: Lowercase hex string (64 chars)
```

### Ed25519 Signatures

**Key Format**:
- Private key: 32 bytes
- Public key: 32 bytes  
- Signature: 64 bytes

**Encoding**: Hex-encoded (lowercase)

## 5. Operations

### generate_keypair()
Returns Ed25519 keypair (private: 32 bytes, public: 32 bytes)

### compute_hash(data)
Returns SHA-256 hash as hex string

### sign_claim(claim, private_key)
1. Validate claim
2. Check 2KB limit
3. Serialize to canonical JSON
4. Sign with Ed25519
5. Return SignedClaim

### verify_claim(signed_claim)
1. Decode hex public key
2. Decode hex signature
3. Serialize claim
4. Verify Ed25519 signature
5. Return boolean

## 6. Size Limits

**Metadata Limit**: 2KB (2048 bytes)

Error message: "Error: Metadata too large. Tip: For large datasets, hash the file locally and anchor the hash instead of the raw data."

## 7. Error Types

```typescript
enum SdkError {
  SerializationError(string),
  SignatureError(string),
  KeyError(string),
  ValidationError(string)
}
```

## 8. Test Vectors

### Vector 1: Basic Claim
```json
{
  "claim": {
    "data": "test_data_123",
    "timestamp": 1704067200
  },
  "canonical_json": "{\"data\":\"test_data_123\",\"timestamp\":1704067200}"
}
```

### Vector 2: Hash
```
Input: "Hello, Provncloud!"
Expected: "7c3e8..." (SHA-256)
```

## 9. WASM Interface

```rust
#[wasm_bindgen]
pub fn wasm_generate_keypair() -> String;

#[wasm_bindgen]
pub fn wasm_compute_hash(data: &[u8]) -> String;

#[wasm_bindgen]
pub fn wasm_sign_claim(claim_json: &str, private_key_hex: &str) -> Result<String, JsValue>;

#[wasm_bindgen]
pub fn wasm_verify_claim(signed_claim_json: &str) -> Result<bool, JsValue>;
```

## 10. Version History

| Spec | SDK | Changes |
|------|-----|---------|
| 1.0 | 0.2.0 | Initial release |

## References

- [RFC 8785 - JCS](https://tools.ietf.org/html/rfc8785)
- [Ed25519](https://ed25519.cr.yp.to/)
- [SHA-256](https://csrc.nist.gov/)
