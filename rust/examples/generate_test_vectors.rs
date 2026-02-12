//! Cross-SDK Interoperability Test Generator
//!
//! This binary generates test vectors that can be used by all SDKs
//! to verify cross-compatibility.

use provn_sdk::{compute_hash, generate_keypair, sign_claim, Claim};
use serde_json::json;

fn main() {
    println!("Generating cross-SDK test vectors...\n");

    // Test Vector 1: Basic claim
    let keypair1 = generate_keypair();
    let claim1 = Claim::new_with_timestamp("test_data_123".to_string(), 1704067200);
    let signed1 = sign_claim(&claim1, &keypair1).unwrap();

    let vector1 = json!({
        "name": "basic_claim",
        "description": "Basic claim with data and timestamp",
        "private_key": hex::encode(keypair1.to_bytes()),
        "public_key": hex::encode(keypair1.verifying_key().as_bytes()),
        "claim": {
            "data": claim1.data,
            "timestamp": claim1.timestamp,
        },
        "canonical_json": r#"{"data":"test_data_123","timestamp":1704067200}"#,
        "expected_signature": signed1.signature,
    });

    // Test Vector 2: Claim with metadata
    let keypair2 = generate_keypair();
    let mut claim2 = Claim::new_with_timestamp("hash_abc123".to_string(), 1704067200);
    claim2.metadata = Some("user_id:12345".to_string());
    let signed2 = sign_claim(&claim2, &keypair2).unwrap();

    let vector2 = json!({
        "name": "claim_with_metadata",
        "description": "Claim with optional metadata field",
        "private_key": hex::encode(keypair2.to_bytes()),
        "public_key": hex::encode(keypair2.verifying_key().as_bytes()),
        "claim": {
            "data": claim2.data,
            "metadata": claim2.metadata,
            "timestamp": claim2.timestamp,
        },
        "canonical_json": r#"{"data":"hash_abc123","metadata":"user_id:12345","timestamp":1704067200}"#,
        "expected_signature": signed2.signature,
    });

    // Test Vector 3: Hash computation
    let hash1 = compute_hash(b"hello");
    let hash2 = compute_hash(b"");
    let hash3 = compute_hash(b"Provncloud SDK");

    let vector3 = json!({
        "name": "hash_computation",
        "description": "SHA-256 hash test vectors",
        "test_cases": [
            {
                "input": "hello",
                "expected_hash": hash1
            },
            {
                "input": "",
                "expected_hash": hash2
            },
            {
                "input": "Provncloud SDK",
                "expected_hash": hash3
            }
        ]
    });

    // Test Vector 4: Cross-sign test (same claim, different keys)
    let keypair_a = generate_keypair();
    let keypair_b = generate_keypair();
    let claim_cross = Claim::new_with_timestamp("cross_test_data".to_string(), 1704067200);

    let signed_by_a = sign_claim(&claim_cross, &keypair_a).unwrap();
    let signed_by_b = sign_claim(&claim_cross, &keypair_b).unwrap();

    let vector4 = json!({
        "name": "cross_sign_same_claim",
        "description": "Same claim signed by two different keys",
        "claim": {
            "data": claim_cross.data,
            "timestamp": claim_cross.timestamp,
        },
        "canonical_json": r#"{"data":"cross_test_data","timestamp":1704067200}"#,
        "signatures": [
            {
                "signer": "key_a",
                "private_key": hex::encode(keypair_a.to_bytes()),
                "public_key": hex::encode(keypair_a.verifying_key().as_bytes()),
                "signature": signed_by_a.signature
            },
            {
                "signer": "key_b",
                "private_key": hex::encode(keypair_b.to_bytes()),
                "public_key": hex::encode(keypair_b.verifying_key().as_bytes()),
                "signature": signed_by_b.signature
            }
        ]
    });

    // Combine all vectors
    let test_vectors = json!({
        "spec_version": "1.0",
        "sdk_version": "0.2.0",
        "generated_at": "2024-01-01T00:00:00Z",
        "test_vectors": [
            vector1,
            vector2,
            vector3,
            vector4
        ]
    });

    // Print as formatted JSON
    println!("{}", serde_json::to_string_pretty(&test_vectors).unwrap());
}
