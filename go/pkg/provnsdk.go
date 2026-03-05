// Package provnsdk provides Provncloud SDK for Go
//
// A privacy-preserving digital signature SDK for anchoring data to
// Arweave AO and Solana blockchains.
package provnsdk

import (
	"crypto/ed25519"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"time"
)

const (
	// MaxPayloadSize is the maximum size of serialized claim in bytes
	MaxPayloadSize = 2048
)

// SDKError represents errors from the SDK
type SDKError struct {
	Type    string
	Message string
}

func (e SDKError) Error() string {
	return fmt.Sprintf("%s: %s", e.Type, e.Message)
}

// Claim represents a statement of truth to be anchored
type Claim struct {
	Data      string `json:"data"`
	Metadata  string `json:"metadata,omitempty"`
	Timestamp uint64 `json:"timestamp"`
}

// SignedClaim wraps a Claim with cryptographic proof
type SignedClaim struct {
	Claim     Claim  `json:"claim"`
	PublicKey string `json:"public_key"`
	Signature string `json:"signature"`
}

// KeyPair represents an Ed25519 keypair
type KeyPair struct {
	PrivateKey ed25519.PrivateKey
	PublicKey  ed25519.PublicKey
}

// GenerateKeypair generates a new Ed25519 keypair
func GenerateKeypair() (*KeyPair, error) {
	pub, priv, err := ed25519.GenerateKey(nil)
	if err != nil {
		return nil, SDKError{Type: "KeyError", Message: err.Error()}
	}

	return &KeyPair{
		PrivateKey: priv,
		PublicKey:  pub,
	}, nil
}

// ComputeHash computes SHA-256 hash of data
func ComputeHash(data []byte) string {
	hash := sha256.Sum256(data)
	return hex.EncodeToString(hash[:])
}

// CreateClaim creates a claim with current timestamp
func CreateClaim(data string, metadata string) *Claim {
	return &Claim{
		Data:      data,
		Timestamp: uint64(time.Now().Unix()),
		Metadata:  metadata,
	}
}

// CreateClaimWithTimestamp creates a claim with explicit timestamp
func CreateClaimWithTimestamp(data string, timestamp uint64, metadata string) (*Claim, error) {
	if len(data) == 0 {
		return nil, SDKError{Type: "ValidationError", Message: "Data field cannot be empty."}
	}

	if timestamp < 1 || timestamp > 32503680000 {
		return nil, SDKError{Type: "ValidationError", Message: "Timestamp out of bounds."}
	}
	return &Claim{
		Data:      data,
		Timestamp: timestamp,
		Metadata:  metadata,
	}, nil
}

// ToSignableBytes serializes claim to canonical JSON bytes
func (c *Claim) ToSignableBytes() ([]byte, error) {
	// Check metadata size limit
	jsonBytes, err := json.Marshal(c)
	if err != nil {
		return nil, SDKError{Type: "SerializationError", Message: err.Error()}
	}

	if len(jsonBytes) > MaxPayloadSize {
		return nil, SDKError{
			Type:    "ValidationError",
			Message: "Error: Payload too large. Tip: For large datasets, hash the file locally and anchor the hash instead of the raw data.",
		}
	}

	return jsonBytes, nil
}

// SignClaim signs a claim with a private key
func SignClaim(claim *Claim, keypair *KeyPair) (*SignedClaim, error) {
	// Get signable bytes
	msg, err := claim.ToSignableBytes()
	if err != nil {
		return nil, err
	}

	// Sign
	signature := ed25519.Sign(keypair.PrivateKey, msg)

	return &SignedClaim{
		Claim:     *claim,
		PublicKey: hex.EncodeToString(keypair.PublicKey),
		Signature: hex.EncodeToString(signature),
	}, nil
}

// VerifyClaim verifies a signed claim
func VerifyClaim(signedClaim *SignedClaim) (bool, error) {
	// Decode public key
	pubKeyBytes, err := hex.DecodeString(signedClaim.PublicKey)
	if err != nil {
		return false, SDKError{Type: "KeyError", Message: fmt.Sprintf("Invalid public key hex: %v", err)}
	}

	if len(pubKeyBytes) != ed25519.PublicKeySize {
		return false, SDKError{Type: "KeyError", Message: fmt.Sprintf("Invalid public key length: expected %d, got %d", ed25519.PublicKeySize, len(pubKeyBytes))}
	}

	// Decode signature
	sigBytes, err := hex.DecodeString(signedClaim.Signature)
	if err != nil {
		return false, SDKError{Type: "SignatureError", Message: fmt.Sprintf("Invalid signature hex: %v", err)}
	}

	// Get signable bytes
	msg, err := signedClaim.Claim.ToSignableBytes()
	if err != nil {
		return false, err
	}

	// Verify
	valid := ed25519.Verify(pubKeyBytes, msg, sigBytes)
	return valid, nil
}

// GetVersion returns the SDK version
func GetVersion() string {
	return "0.2.0"
}

// ExportPrivateKey exports private key seed as hex string (32 bytes).
// WARNING: This seed must be heavily guarded. It grants full control over the identity.
func (kp *KeyPair) ExportPrivateKey() string {
	// Go's ed25519.PrivateKey is 64 bytes, but we only export the seed (first 32 bytes)
	return hex.EncodeToString(kp.PrivateKey[0:32])
}

// ExportPublicKey exports public key as hex string
func (kp *KeyPair) ExportPublicKey() string {
	return hex.EncodeToString(kp.PublicKey)
}

// ImportKeypair imports a keypair from hex strings
// Note: In Go, ed25519.PrivateKey is 64 bytes (seed + public key)
// But we store only the 32-byte seed, so we reconstruct the full key
func ImportKeypair(privateKeyHex string, publicKeyHex string) (*KeyPair, error) {
	seedBytes, err := hex.DecodeString(privateKeyHex)
	if err != nil {
		return nil, SDKError{Type: "KeyError", Message: fmt.Sprintf("Invalid private key hex: %v", err)}
	}

	pubBytes, err := hex.DecodeString(publicKeyHex)
	if err != nil {
		return nil, SDKError{Type: "KeyError", Message: fmt.Sprintf("Invalid public key hex: %v", err)}
	}

	if len(seedBytes) != 32 {
		return nil, SDKError{Type: "KeyError", Message: fmt.Sprintf("Invalid seed length: expected 32, got %d", len(seedBytes))}
	}

	if len(pubBytes) != 32 {
		return nil, SDKError{Type: "KeyError", Message: fmt.Sprintf("Invalid public key length: expected 32, got %d", len(pubBytes))}
	}

	// Go's ed25519.PrivateKey is seed (32 bytes) + public key (32 bytes)
	// We must derive the public key from the seed to ensure it matches the provided public key
	derivedPrivateKey := ed25519.NewKeyFromSeed(seedBytes)

	// Validate that the provided public key matches the derived public key
	if hex.EncodeToString(derivedPrivateKey.Public().(ed25519.PublicKey)) != hex.EncodeToString(pubBytes) {
		return nil, SDKError{Type: "KeyError", Message: "Provided public key does not match seed"}
	}

	return &KeyPair{
		PrivateKey: derivedPrivateKey,
		PublicKey:  pubBytes,
	}, nil
}
