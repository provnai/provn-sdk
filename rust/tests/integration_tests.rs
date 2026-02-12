//! Integration tests for Provncloud SDK
//!
//! These tests verify end-to-end workflows and cross-compatibility.

use provn_sdk::{compute_hash, generate_keypair, sign_claim, verify_claim, Claim, SignedClaim};

/// Test complete workflow: generate key → sign claim → verify
#[test]
fn test_end_to_end_workflow() {
    // 1. Generate keypair
    let keypair = generate_keypair();
    let public_key = hex::encode(keypair.verifying_key().as_bytes());

    // 2. Create claim
    let data = "test_data_for_integration";
    let hash = compute_hash(data.as_bytes());
    let claim = Claim::new(hash.clone());

    // 3. Sign claim
    let signed = sign_claim(&claim, &keypair).expect("Signing should succeed");

    // 4. Verify signature
    assert!(verify_claim(&signed).expect("Verification should not fail"));

    // 5. Verify claim contents
    assert_eq!(signed.claim.data, hash);
    assert_eq!(signed.public_key, public_key);
    assert!(!signed.signature.is_empty());
}

/// Test serialization roundtrip
#[test]
fn test_serialization_roundtrip() {
    let keypair = generate_keypair();
    let claim = Claim::new("test_hash".to_string());
    let signed = sign_claim(&claim, &keypair).unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&signed).expect("Serialization should succeed");

    // Deserialize back
    let deserialized: SignedClaim =
        serde_json::from_str(&json).expect("Deserialization should succeed");

    // Verify it still works
    assert!(verify_claim(&deserialized).expect("Verification should succeed"));

    // Verify contents match
    assert_eq!(deserialized.claim.data, signed.claim.data);
    assert_eq!(deserialized.public_key, signed.public_key);
    assert_eq!(deserialized.signature, signed.signature);
}

/// Test hash computation produces consistent results
#[test]
fn test_hash_consistency() {
    let data = b"consistent test data";

    let hash1 = compute_hash(data);
    let hash2 = compute_hash(data);

    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 64); // SHA-256 hex is 64 chars
}

/// Test known SHA-256 hash values
#[test]
fn test_known_hashes() {
    // Test vector: "hello"
    let hash = compute_hash(b"hello");
    assert_eq!(
        hash,
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );

    // Test vector: empty string
    let hash = compute_hash(b"");
    assert_eq!(
        hash,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

/// Test claim with metadata at exactly 2KB limit
#[test]
fn test_metadata_at_2kb_limit() {
    let keypair = generate_keypair();

    // Calculate exact size needed
    let base_claim = Claim::new_with_timestamp("data".to_string(), 1234567890);
    let base_size = base_claim.to_signable_bytes().unwrap().len();

    // Metadata should be 2048 - base_size to hit exactly 2KB
    let metadata_size = 2048 - base_size;
    let metadata = "a".repeat(metadata_size);

    let mut claim = Claim::new_with_timestamp("data".to_string(), 1234567890);
    claim.metadata = Some(metadata);

    // Should succeed at exactly 2KB
    let signed = sign_claim(&claim, &keypair).expect("Should sign at exactly 2KB");
    assert!(verify_claim(&signed).unwrap());
}

/// Test metadata exceeding 2KB limit
#[test]
fn test_metadata_exceeds_2kb_limit() {
    let keypair = generate_keypair();

    let mut claim = Claim::new_with_timestamp("data".to_string(), 1234567890);
    claim.metadata = Some("a".repeat(3000)); // Way over limit

    let result = sign_claim(&claim, &keypair);
    assert!(result.is_err());
}

/// Test tampered claim fails verification
#[test]
fn test_tamper_detection_integration() {
    let keypair = generate_keypair();
    let claim = Claim::new("original_data".to_string());
    let mut signed = sign_claim(&claim, &keypair).unwrap();

    // Tamper with data
    signed.claim.data = "tampered_data".to_string();

    // Verification should fail
    let result = verify_claim(&signed);
    assert!(result.is_err() || result.unwrap() == false);
}

/// Test tampered signature fails verification
#[test]
fn test_signature_tamper_detection() {
    let keypair = generate_keypair();
    let claim = Claim::new("data".to_string());
    let mut signed = sign_claim(&claim, &keypair).unwrap();

    // Tamper with signature (flip last char)
    let last_char = signed.signature.pop().unwrap();
    let flipped = if last_char == 'a' { 'b' } else { 'a' };
    signed.signature.push(flipped);

    // Verification should fail
    let result = verify_claim(&signed);
    assert!(result.is_err() || result.unwrap() == false);
}

/// Test invalid public key format
#[test]
fn test_invalid_public_key() {
    let keypair = generate_keypair();
    let claim = Claim::new("data".to_string());
    let mut signed = sign_claim(&claim, &keypair).unwrap();

    // Invalid hex
    signed.public_key = "not_valid_hex!!!".to_string();

    let result = verify_claim(&signed);
    assert!(result.is_err());
}

/// Test wrong public key
#[test]
fn test_wrong_public_key() {
    let keypair1 = generate_keypair();
    let keypair2 = generate_keypair();

    let claim = Claim::new("data".to_string());
    let mut signed = sign_claim(&claim, &keypair1).unwrap();

    // Use different keypair's public key
    signed.public_key = hex::encode(keypair2.verifying_key().as_bytes());

    // Verification should fail
    let result = verify_claim(&signed);
    assert!(result.is_err() || result.unwrap() == false);
}

/// Test multiple claims with same keypair
#[test]
fn test_multiple_claims_same_key() {
    let keypair = generate_keypair();

    for i in 0..10 {
        let claim = Claim::new(format!("claim_{}", i));
        let signed = sign_claim(&claim, &keypair).unwrap();
        assert!(verify_claim(&signed).unwrap());
    }
}

/// Test claims with different keypairs
#[test]
fn test_different_keypairs() {
    for _ in 0..5 {
        let keypair = generate_keypair();
        let claim = Claim::new("test".to_string());
        let signed = sign_claim(&claim, &keypair).unwrap();
        assert!(verify_claim(&signed).unwrap());
    }
}

/// Test unicode data handling
#[test]
fn test_unicode_data() {
    let keypair = generate_keypair();
    let claim = Claim::new("Hello 世界 🌍".to_string());
    let signed = sign_claim(&claim, &keypair).unwrap();
    assert!(verify_claim(&signed).unwrap());
}

/// Test large data hash (simulating file hash)
#[test]
fn test_large_data_hash() {
    let keypair = generate_keypair();

    // Simulate a file hash (64 chars)
    let file_hash = "a".repeat(64);
    let claim = Claim::new(file_hash);
    let signed = sign_claim(&claim, &keypair).unwrap();
    assert!(verify_claim(&signed).unwrap());
}

/// Test claim without metadata
#[test]
fn test_claim_without_metadata() {
    let keypair = generate_keypair();
    let claim = Claim::new("data_without_metadata".to_string());

    assert!(claim.metadata.is_none());

    let signed = sign_claim(&claim, &keypair).unwrap();
    assert!(verify_claim(&signed).unwrap());
}

/// Test timestamp boundaries
#[test]
fn test_timestamp_boundaries() {
    let keypair = generate_keypair();

    // Unix epoch
    let claim = Claim::new_with_timestamp("data".to_string(), 0);
    let signed = sign_claim(&claim, &keypair).unwrap();
    assert!(verify_claim(&signed).unwrap());

    // Future timestamp (should still work)
    let claim = Claim::new_with_timestamp("data".to_string(), 4102444800); // Year 2100
    let signed = sign_claim(&claim, &keypair).unwrap();
    assert!(verify_claim(&signed).unwrap());
}

/// Test canonical JSON ordering
#[test]
fn test_canonical_json_determinism() {
    let claim1 = Claim {
        data: "test".to_string(),
        metadata: Some("meta".to_string()),
        timestamp: 123,
    };

    let claim2 = Claim {
        timestamp: 123,
        data: "test".to_string(),
        metadata: Some("meta".to_string()),
    };

    // Both should serialize to same canonical JSON
    let json1 = serde_json::to_string(&claim1).unwrap();
    let json2 = serde_json::to_string(&claim2).unwrap();

    assert_eq!(json1, json2);
    assert_eq!(
        json1,
        r#"{"data":"test","metadata":"meta","timestamp":123}"#
    );
}
