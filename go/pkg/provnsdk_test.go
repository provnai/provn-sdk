package provnsdk

import (
	"strings"
	"testing"
)

func TestGenerateKeypair(t *testing.T) {
	kp, err := GenerateKeypair()
	if err != nil {
		t.Fatalf("Failed to generate keypair: %v", err)
	}

	if kp.PrivateKey == nil {
		t.Error("Private key is nil")
	}

	if kp.PublicKey == nil {
		t.Error("Public key is nil")
	}

	// Check hex exports
	privHex := kp.ExportPrivateKey()
	pubHex := kp.ExportPublicKey()

	// ExportPrivateKey now returns only the seed (32 bytes = 64 hex chars)
	if len(privHex) != 64 {
		t.Errorf("Expected private key hex length 64, got %d", len(privHex))
	}

	if len(pubHex) != 64 {
		t.Errorf("Expected public key hex length 64, got %d", len(pubHex))
	}
}

func TestComputeHash(t *testing.T) {
	hash := ComputeHash([]byte("hello"))

	expected := "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
	if hash != expected {
		t.Errorf("Expected hash %s, got %s", expected, hash)
	}

	if len(hash) != 64 {
		t.Errorf("Expected hash length 64, got %d", len(hash))
	}
}

func TestCreateClaim(t *testing.T) {
	claim := CreateClaim("test_data", "test_metadata")

	if claim.Data != "test_data" {
		t.Errorf("Expected data 'test_data', got '%s'", claim.Data)
	}

	if claim.Metadata == nil || *claim.Metadata != "test_metadata" {
		t.Errorf("Expected metadata 'test_metadata', got '%v'", claim.Metadata)
	}

	if claim.Timestamp == 0 {
		t.Error("Timestamp should not be 0")
	}
}

func TestCreateClaimPreservesEmptyMetadata(t *testing.T) {
	claim := CreateClaim("test_data", "")

	if claim.Metadata == nil {
		t.Fatal("Expected metadata pointer to be present for explicit empty string")
	}

	if *claim.Metadata != "" {
		t.Errorf("Expected empty metadata string, got %q", *claim.Metadata)
	}
}

func TestCreateClaimWithoutMetadata(t *testing.T) {
	claim := CreateClaimWithoutMetadata("test_data")

	if claim.Metadata != nil {
		t.Errorf("Expected metadata to be omitted, got %v", claim.Metadata)
	}
}

func TestSignAndVerifyClaim(t *testing.T) {
	kp, err := GenerateKeypair()
	if err != nil {
		t.Fatalf("Failed to generate keypair: %v", err)
	}

	claim, _ := CreateClaimWithTimestamp("test_data", 1234567890, nil)

	signed, err := SignClaim(claim, kp)
	if err != nil {
		t.Fatalf("Failed to sign claim: %v", err)
	}

	if signed.PublicKey != kp.ExportPublicKey() {
		t.Error("Public key mismatch")
	}

	if len(signed.Signature) != 128 {
		t.Errorf("Expected signature length 128, got %d", len(signed.Signature))
	}

	// Verify
	valid, err := VerifyClaim(signed)
	if err != nil {
		t.Fatalf("Verification error: %v", err)
	}

	if !valid {
		t.Error("Claim should be valid")
	}
}

func TestVerifyTamperedClaim(t *testing.T) {
	kp, _ := GenerateKeypair()
	claim, _ := CreateClaimWithTimestamp("original_data", 1234567890, nil)

	signed, _ := SignClaim(claim, kp)

	// Tamper with data
	signed.Claim.Data = "tampered_data"

	valid, err := VerifyClaim(signed)
	if err != nil {
		t.Fatalf("Verification error: %v", err)
	}

	if valid {
		t.Error("Tampered claim should be invalid")
	}
}

func TestVerifyTamperedSignature(t *testing.T) {
	kp, _ := GenerateKeypair()
	claim, _ := CreateClaimWithTimestamp("test_data", 1234567890, nil)

	signed, _ := SignClaim(claim, kp)

	// Tamper with signature by flipping the last character
	lastChar := signed.Signature[len(signed.Signature)-1]
	flipped := "a"
	if lastChar == 'a' {
		flipped = "b"
	}
	signed.Signature = signed.Signature[:len(signed.Signature)-1] + flipped

	valid, err := VerifyClaim(signed)
	if err != nil {
		t.Fatalf("Verification error: %v", err)
	}

	if valid {
		t.Error("Claim with tampered signature should be invalid")
	}
}

func TestVerifyWrongPublicKey(t *testing.T) {
	kp1, _ := GenerateKeypair()
	kp2, _ := GenerateKeypair()

	claim, _ := CreateClaimWithTimestamp("test_data", 1234567890, nil)
	signed, _ := SignClaim(claim, kp1)

	// Use different public key
	signed.PublicKey = kp2.ExportPublicKey()

	valid, err := VerifyClaim(signed)
	if err != nil {
		t.Fatalf("Verification error: %v", err)
	}

	if valid {
		t.Error("Claim with wrong public key should be invalid")
	}
}

func TestCanonicalJSON(t *testing.T) {
	meta := "meta"
	claim1 := &Claim{
		Data:      "test",
		Metadata:  &meta,
		Timestamp: 123,
	}

	claim2 := &Claim{
		Timestamp: 123,
		Data:      "test",
		Metadata:  &meta,
	}

	json1, err := claim1.ToSignableBytes()
	if err != nil {
		t.Fatalf("Failed to serialize claim1: %v", err)
	}
	json2, err := claim2.ToSignableBytes()
	if err != nil {
		t.Fatalf("Failed to serialize claim2: %v", err)
	}

	if string(json1) != string(json2) {
		t.Error("Canonical JSON should be deterministic")
	}

	expected := `{"data":"test","metadata":"meta","timestamp":123}`
	if string(json1) != expected {
		t.Errorf("Expected %s, got %s", expected, string(json1))
	}
}

func TestMetadataSizeLimit(t *testing.T) {
	kp, _ := GenerateKeypair()

	// Create claim with metadata exceeding 2KB
	largeStr := strings.Repeat("a", 3000)

	claim, _ := CreateClaimWithTimestamp("data", 1234567890, &largeStr)

	_, err := SignClaim(claim, kp)
	if err == nil {
		t.Error("Should fail with metadata exceeding 2KB")
	}
}

func TestImportExportKeypair(t *testing.T) {
	kp1, _ := GenerateKeypair()

	privHex := kp1.ExportPrivateKey()
	pubHex := kp1.ExportPublicKey()

	kp2, err := ImportKeypair(privHex, pubHex)
	if err != nil {
		t.Fatalf("Failed to import keypair: %v", err)
	}

	if string(kp1.PrivateKey) != string(kp2.PrivateKey) {
		t.Error("Private keys don't match")
	}

	if string(kp1.PublicKey) != string(kp2.PublicKey) {
		t.Error("Public keys don't match")
	}
}

func TestGetVersion(t *testing.T) {
	version := GetVersion()
	if version == "" {
		t.Error("Version should not be empty")
	}
}

func TestParseSignedClaimStrictRejectsUnknownFields(t *testing.T) {
	// Valid claim JSON but with an injected extra field
	maliciousJSON := `{
		"claim": {"data": "test", "timestamp": 1234567890, "injected": "evil"},
		"public_key": "aabbcc",
		"signature": "ddeeff"
	}`

	_, err := ParseSignedClaimStrict(maliciousJSON)
	if err == nil {
		t.Error("Expected error for unknown field 'injected', got nil")
	}
	t.Logf("Correctly rejected: %v", err)
}
