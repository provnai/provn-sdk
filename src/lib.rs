#![cfg_attr(not(feature = "std"), no_std)]

//! # Provncloud SDK
//!
//! A high-performance, universal cryptographic engine for signing and verifying data claims.
//!
//! This crate provides a lightweight, `no_std` compatible implementation for generating
//! verifiable audit trails. It is designed to be compatible with resource-constrained
//! environments like Solana programs and Arweave AO processes, while maintaining 
//! strict [JCS (RFC 8785)](https://rfc-editor.org/rfc/rfc8785) compliance for
//! cross-platform interoperability.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

/// Errors encountered during SDK operations.
#[derive(Debug)]
pub enum SdkError {
    /// Error occurred during JSON serialization or deserialization.
    SerializationError(String),
    /// Error occurred during cryptographic signature generation or verification.
    SignatureError(String),
    /// Error occurred due to invalid key format or length.
    KeyError(String),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdkError::SerializationError(e) => write!(f, "Serialization failed: {}", e),
            SdkError::SignatureError(e) => write!(f, "Invalid signature: {}", e),
            SdkError::KeyError(e) => write!(f, "Key format error: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SdkError {}

impl From<serde_json::Error> for SdkError {
    fn from(e: serde_json::Error) -> Self {
        SdkError::SerializationError(e.to_string())
    }
}

impl From<ed25519_dalek::SignatureError> for SdkError {
    fn from(e: ed25519_dalek::SignatureError) -> Self {
        SdkError::SignatureError(e.to_string())
    }
}

pub type Result<T> = core::result::Result<T, SdkError>;

/// A Claim representing a statement of truth to be anchored.
/// Fields are ordered alphabetically to ensure "Canonical JSON" (JCS - RFC 8785)
/// compliance when using deterministic serialization.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Claim {
    /// The actual data being claimed (e.g., "AI Model v1.0 Accuracy: 98%")
    pub data: String,
    /// Optional metadata or context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// Timestamp of the claim (UTC seconds)
    pub timestamp: u64,
}

/// A SignedClaim wraps the claim with its signature and public key.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedClaim {
    /// The original claim
    pub claim: Claim,
    /// The public key of the signer (Hex encoded)
    pub public_key: String,
    /// The signature of the serialized claim (Hex encoded)
    pub signature: String,
}

impl Claim {
    /// Create a new claim with the current system time (requires "std")
    #[cfg(feature = "std")]
    pub fn new(data: String) -> Self {
        Self {
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            metadata: None,
        }
    }

    /// Create a new claim with a provided timestamp (useful for no-std)
    pub fn new_with_timestamp(data: String, timestamp: u64) -> Self {
        Self {
            data,
            timestamp,
            metadata: None,
        }
    }

    /// Canonical serialization for signing (Sorted keys, no whitespace)
    /// This follows JCS (RFC 8785) logic by relying on struct field ordering.
    pub fn to_signable_bytes(&self) -> Result<Vec<u8>> {
        // Enforce canonical JSON (no whitespace, sorted keys via struct order)
        let json = serde_json::to_string(self)?;
        Ok(json.into_bytes())
    }
}

/// Compute a SHA-256 hash of a byte slice for "Hash-Only" forensics.
pub fn compute_hash(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Generate a new random keypair (requires "std" for entropy)
///
/// # Example
/// ```
/// use provn_sdk::generate_keypair;
/// let key = generate_keypair();
/// ```
#[cfg(feature = "std")]
pub fn generate_keypair() -> SigningKey {
    use rand::rngs::OsRng;
    SigningKey::generate(&mut OsRng)
}

/// Sign a claim with a private key
///
/// # Example
/// ```
/// use provn_sdk::{Claim, sign_claim, generate_keypair};
/// let key = generate_keypair();
/// let claim = Claim::new("Test Claim".to_string());
/// let signed = sign_claim(&claim, &key).unwrap();
/// ```
pub fn sign_claim(claim: &Claim, key: &SigningKey) -> Result<SignedClaim> {
    let bytes = claim.to_signable_bytes()?;
    let signature = key.sign(&bytes);

    Ok(SignedClaim {
        claim: claim.clone(),
        public_key: hex::encode(key.verifying_key().as_bytes()),
        signature: hex::encode(signature.to_bytes()),
    })
}

/// Verify a signed claim
///
/// # Example
/// ```
/// use provn_sdk::{Claim, sign_claim, verify_claim, generate_keypair};
/// let key = generate_keypair();
/// let claim = Claim::new("Test Claim".to_string());
/// let signed = sign_claim(&claim, &key).unwrap();
/// assert!(verify_claim(&signed).unwrap());
/// ```
pub fn verify_claim(signed_claim: &SignedClaim) -> Result<bool> {
    // 1. Decode Public Key
    let pk_bytes = hex::decode(&signed_claim.public_key)
        .map_err(|e| SdkError::KeyError(format!("Invalid Hex Public Key: {}", e)))?;
    let pk = VerifyingKey::from_bytes(
        pk_bytes
            .as_slice()
            .try_into()
            .map_err(|_| SdkError::KeyError("Invalid Key Length".into()))?,
    )?;

    // 2. Decode Signature
    let sig_bytes = hex::decode(&signed_claim.signature)
        .map_err(|e| SdkError::KeyError(format!("Invalid Hex Signature: {}", e)))?;
    let sig = Signature::from_bytes(
        sig_bytes
            .as_slice()
            .try_into()
            .map_err(|_| SdkError::KeyError("Invalid Signature Length".into()))?,
    );

    // 3. Reconstruct Signable Bytes
    let msg_bytes = signed_claim.claim.to_signable_bytes()?;

    // 4. Verify
    pk.verify(&msg_bytes, &sig)?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify_flow() {
        let key = SigningKey::from_bytes(&[0u8; 32]);
        let claim = Claim::new_with_timestamp("Hello World".to_string(), 123456789);

        let signed = sign_claim(&claim, &key).expect("Sign failed");

        // Verify locally
        let valid = verify_claim(&signed).expect("Verify failed");
        assert!(valid);
    }

    #[test]
    fn test_canonical_json_order() {
        let claim = Claim {
            data: "test".to_string(),
            metadata: Some("meta".to_string()),
            timestamp: 123,
        };
        let json = serde_json::to_string(&claim).unwrap();
        // data comes before metadata comes before timestamp
        assert_eq!(json, r#"{"data":"test","metadata":"meta","timestamp":123}"#);
    }

    #[test]
    fn test_tamper_detection() {
        let key = SigningKey::from_bytes(&[0u8; 32]);
        let claim = Claim::new_with_timestamp("Sensitive Data".to_string(), 123456789);

        let mut signed = sign_claim(&claim, &key).expect("Sign failed");

        // Tamper with data
        signed.claim.data = "Tampered Data".to_string();

        let result = verify_claim(&signed);
        assert!(result.is_err());
    }
}
