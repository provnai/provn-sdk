//! Provncloud SDK WebAssembly Bindings
//!
//! This crate provides WebAssembly bindings for the Provncloud SDK,
//! enabling browser-based and Node.js usage.

mod utils;

use ed25519_dalek::SigningKey;
use provn_sdk::{compute_hash, generate_keypair, sign_claim, verify_claim, Claim};
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();
}

/// Generate a new Ed25519 keypair
///
/// Returns a JSON object: `{"private_key": "hex", "public_key": "hex"}`
#[wasm_bindgen]
pub fn wasm_generate_keypair() -> Result<String, JsValue> {
    let keypair = generate_keypair();
    let public_key = hex::encode(keypair.verifying_key().as_bytes());
    let private_key = hex::encode(keypair.to_bytes());

    let result = serde_json::json!({
        "private_key": private_key,
        "public_key": public_key,
    });

    Ok(result.to_string())
}

/// Compute SHA-256 hash of data
///
/// Input: byte array
/// Returns: hex string
#[wasm_bindgen]
pub fn wasm_compute_hash(data: &[u8]) -> String {
    compute_hash(data)
}

/// Sign a claim
///
/// Parameters:
/// - `claim_json`: JSON string of Claim object
/// - `private_key_hex`: Hex-encoded private key
///
/// Returns: JSON string of SignedClaim or error
#[wasm_bindgen]
pub fn wasm_sign_claim(claim_json: &str, private_key_hex: &str) -> Result<String, JsValue> {
    // Parse claim
    let claim: Claim = serde_json::from_str(claim_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse claim: {}", e)))?;

    // Decode private key
    let key_bytes = hex::decode(private_key_hex)
        .map_err(|e| JsValue::from_str(&format!("Invalid private key hex: {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(JsValue::from_str("Private key must be 32 bytes"));
    }

    let key_array: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| JsValue::from_str("Failed to convert key to array"))?;

    let signing_key = SigningKey::from_bytes(&key_array);

    // Sign claim
    let signed = sign_claim(&claim, &signing_key)
        .map_err(|e| JsValue::from_str(&format!("Signing failed: {}", e)))?;

    // Serialize result
    serde_json::to_string(&signed)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize signed claim: {}", e)))
}

/// Verify a signed claim
///
/// Parameters:
/// - `signed_claim_json`: JSON string of SignedClaim
///
/// Returns: true if valid, false otherwise
#[wasm_bindgen]
pub fn wasm_verify_claim(signed_claim_json: &str) -> Result<bool, JsValue> {
    // Parse signed claim
    let signed: provn_sdk::SignedClaim = serde_json::from_str(signed_claim_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse signed claim: {}", e)))?;

    // Verify
    verify_claim(&signed).map_err(|e| JsValue::from_str(&format!("Verification failed: {}", e)))
}

/// Get SDK version
#[wasm_bindgen]
pub fn wasm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Create a claim with current timestamp
///
/// Uses JavaScript's Date.now() for timestamp
#[wasm_bindgen]
pub fn wasm_create_claim(data: &str, metadata: Option<String>) -> Result<String, JsValue> {
    // Get current timestamp from JS safely without fp64 precision loss
    let timestamp = (js_sys::Math::trunc(js_sys::Date::now() / 1000.0)) as u64;

    let claim = Claim {
        data: data.to_string(),
        timestamp,
        metadata,
    };

    serde_json::to_string(&claim)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize claim: {}", e)))
}

/// Create a claim with explicit timestamp
#[wasm_bindgen]
pub fn wasm_create_claim_with_timestamp(
    data: &str,
    timestamp: u64,
    metadata: Option<String>,
) -> Result<String, JsValue> {
    let claim = Claim {
        data: data.to_string(),
        timestamp,
        metadata,
    };

    serde_json::to_string(&claim)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize claim: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_wasm_generate_keypair() {
        let result = wasm_generate_keypair().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(parsed["private_key"].as_str().unwrap().len() == 64);
        assert!(parsed["public_key"].as_str().unwrap().len() == 64);
    }

    #[wasm_bindgen_test]
    fn test_wasm_compute_hash() {
        let hash = wasm_compute_hash(b"hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[wasm_bindgen_test]
    fn test_wasm_sign_and_verify() {
        // Generate key
        let key_json = wasm_generate_keypair().unwrap();
        let key: serde_json::Value = serde_json::from_str(&key_json).unwrap();
        let private_key = key["private_key"].as_str().unwrap();

        // Create claim
        let claim_json = wasm_create_claim_with_timestamp("test_data", 1234567890, None).unwrap();

        // Sign
        let signed_json = wasm_sign_claim(&claim_json, private_key).unwrap();

        // Verify
        let is_valid = wasm_verify_claim(&signed_json).unwrap();
        assert!(is_valid);
    }
}
