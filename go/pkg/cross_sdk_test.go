package provnsdk

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"testing"
)

// TestVectors represents the test vectors file structure
type TestVectors struct {
	SpecVersion string       `json:"spec_version"`
	SDKVersion  string       `json:"sdk_version"`
	GeneratedAt string       `json:"generated_at"`
	Vectors     []TestVector `json:"test_vectors"`
}

// TestVector represents a single test vector
type TestVector struct {
	Name          string          `json:"name"`
	Description   string          `json:"description"`
	PrivateKey    string          `json:"private_key"`
	PublicKey     string          `json:"public_key"`
	Claim         *Claim          `json:"claim"`
	CanonicalJSON string          `json:"canonical_json"`
	ExpectedSig   string          `json:"expected_signature"`
	TestCases     []HashTestCase  `json:"test_cases"`
	Signatures    []SignatureInfo `json:"signatures"`
}

// HashTestCase represents a hash test case
type HashTestCase struct {
	Input        string `json:"input"`
	ExpectedHash string `json:"expected_hash"`
}

// SignatureInfo represents signature information
type SignatureInfo struct {
	Signer     string `json:"signer"`
	PrivateKey string `json:"private_key"`
	PublicKey  string `json:"public_key"`
	Signature  string `json:"signature"`
}

// TestCrossSDKBasicClaim verifies Rust-generated basic claim in Go
func TestCrossSDKBasicClaim(t *testing.T) {
	vectors := loadTestVectors(t)

	for _, vector := range vectors.Vectors {
		if vector.Name != "basic_claim" {
			continue
		}

		t.Logf("Testing: %s - %s", vector.Name, vector.Description)

		// Import the keypair from Rust
		kp, err := ImportKeypair(vector.PrivateKey, vector.PublicKey)
		if err != nil {
			t.Fatalf("Failed to import keypair: %v", err)
		}

		// Verify the claim
		if vector.Claim == nil {
			t.Fatal("Claim is nil")
		}

		// Sign the claim in Go
		signed, err := SignClaim(vector.Claim, kp)
		if err != nil {
			t.Fatalf("Failed to sign claim: %v", err)
		}

		// The signature will be different due to randomness in key generation
		// But we can verify both signatures
		t.Logf("Go signature: %s", signed.Signature)
		t.Logf("Rust signature: %s", vector.ExpectedSig)

		// Verify our Go signature
		valid, err := VerifyClaim(signed)
		if err != nil {
			t.Fatalf("Verification error: %v", err)
		}
		if !valid {
			t.Error("Go-generated signature should be valid")
		}

		// Create a SignedClaim with Rust signature and verify
		rustSigned := &SignedClaim{
			Claim:     *vector.Claim,
			PublicKey: vector.PublicKey,
			Signature: vector.ExpectedSig,
		}

		valid, err = VerifyClaim(rustSigned)
		if err != nil {
			t.Fatalf("Rust signature verification error: %v", err)
		}
		if !valid {
			t.Error("Rust-generated signature should be valid in Go")
		}

		t.Log("✓ Cross-SDK basic claim test passed!")
	}
}

// TestCrossSDKHashComputation verifies hash computation matches Rust
func TestCrossSDKHashComputation(t *testing.T) {
	vectors := loadTestVectors(t)

	for _, vector := range vectors.Vectors {
		if vector.Name != "hash_computation" {
			continue
		}

		t.Logf("Testing: %s - %s", vector.Name, vector.Description)

		for _, tc := range vector.TestCases {
			goHash := ComputeHash([]byte(tc.Input))

			if goHash != tc.ExpectedHash {
				t.Errorf("Hash mismatch for input '%s': Go=%s, Rust=%s",
					tc.Input, goHash, tc.ExpectedHash)
			} else {
				t.Logf("✓ Hash matches for input '%s'", tc.Input)
			}
		}
	}
}

// TestCrossSDKMetadataClaim verifies Rust claim with metadata in Go
func TestCrossSDKMetadataClaim(t *testing.T) {
	vectors := loadTestVectors(t)

	for _, vector := range vectors.Vectors {
		if vector.Name != "claim_with_metadata" {
			continue
		}

		t.Logf("Testing: %s - %s", vector.Name, vector.Description)

		// Verify canonical JSON matches
		canonicalBytes, err := vector.Claim.ToSignableBytes()
		if err != nil {
			t.Fatalf("Failed to get signable bytes: %v", err)
		}

		if string(canonicalBytes) != vector.CanonicalJSON {
			t.Errorf("Canonical JSON mismatch:\nGot:      %s\nExpected: %s",
				string(canonicalBytes), vector.CanonicalJSON)
		}

		// Verify Rust signature in Go
		rustSigned := &SignedClaim{
			Claim:     *vector.Claim,
			PublicKey: vector.PublicKey,
			Signature: vector.ExpectedSig,
		}

		valid, err := VerifyClaim(rustSigned)
		if err != nil {
			t.Fatalf("Rust signature verification error: %v", err)
		}
		if !valid {
			t.Error("Rust signature with metadata should be valid in Go")
		}

		t.Log("✓ Cross-SDK metadata claim test passed!")
	}
}

// TestCrossSDKSameClaim verifies same claim signed by different SDKs
func TestCrossSDKSameClaim(t *testing.T) {
	vectors := loadTestVectors(t)

	for _, vector := range vectors.Vectors {
		if vector.Name != "cross_sign_same_claim" {
			continue
		}

		t.Logf("Testing: %s - %s", vector.Name, vector.Description)

		// Verify both Rust signatures work in Go
		for _, sigInfo := range vector.Signatures {
			signed := &SignedClaim{
				Claim:     *vector.Claim,
				PublicKey: sigInfo.PublicKey,
				Signature: sigInfo.Signature,
			}

			valid, err := VerifyClaim(signed)
			if err != nil {
				t.Fatalf("Verification error for %s: %v", sigInfo.Signer, err)
			}
			if !valid {
				t.Errorf("Signature from %s should be valid", sigInfo.Signer)
			} else {
				t.Logf("✓ Signature from %s is valid", sigInfo.Signer)
			}
		}

		t.Log("✓ Cross-SDK same claim test passed!")
	}
}

// loadTestVectors loads test vectors from JSON file
func loadTestVectors(t *testing.T) *TestVectors {
	data, err := os.ReadFile("../../spec/test-vectors.json")
	if err != nil {
		t.Fatalf("Failed to read test vectors: %v", err)
	}

	var vectors TestVectors
	if err := json.Unmarshal(data, &vectors); err != nil {
		t.Fatalf("Failed to parse test vectors: %v", err)
	}

	return &vectors
}

// Helper to decode hex for testing
func mustDecodeHex(s string) []byte {
	b, err := hex.DecodeString(s)
	if err != nil {
		panic(err)
	}
	return b
}
