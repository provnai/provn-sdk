package main

import (
	"fmt"
	"log"

	provnsdk "github.com/provnai/provn-sdk/go/pkg"
)

// Provncloud Signing Example
//
// This example demonstrates the standard workflow for a "Sovereign" claim:
// 1. Create a secure digital identity (Local Keypair)
// 2. Prepare a statement of truth (Claim)
// 3. Sign the claim locally (Raw data remains local)
// 4. Verify the integrity and authorship
func main() {
	fmt.Println("--- PROVNCLOUD SDK (Go) ---")

	// 1. SECURE IDENTITY: Generate a unique Ed25519 keypair
	keypair, err := provnsdk.GenerateKeypair()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("[1] Identity Generated: ed25519:%s\n", keypair.ExportPublicKey())

	// 2. TRUTH STATEMENT: Define the data to be anchored.
	// For large files or privacy-sensitive data, we compute a local hash first.
	sensitiveData := []byte("Internal Audit Memo: #1234 - High Priority Security Patch applied.")
	assetHash := provnsdk.ComputeHash(sensitiveData)
	fmt.Printf("[2] Prepared Claim (Asset Hash): \"%s\"\n", assetHash)

	// Create claim with current timestamp
	claim := provnsdk.CreateClaimWithoutMetadata(assetHash)
	fmt.Printf("    Timestamp: %d\n", claim.Timestamp)

	// 3. SECURE SIGNING: Sign the claim locally
	// This produces a SignedClaim object containing the original data,
	// the public key, and the cryptographic signature.
	signedClaim, err := provnsdk.SignClaim(claim, keypair)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("[3] Local Signature: %s\n", signedClaim.Signature)

	// 4. INDEPENDENT VERIFICATION: Confirm the claim is authentic
	// This can be done by anybody with the public key,
	// without needing to contact the Provncloud API.
	isValid, err := provnsdk.VerifyClaim(signedClaim)
	if err != nil {
		log.Fatal(err)
	}

	if isValid {
		fmt.Println("[4] Verification: SUCCESS (Sovereign Guarantee Intact)")
	} else {
		fmt.Println("[4] Verification: FAILED (Data Tampering Detected)")
	}
}
