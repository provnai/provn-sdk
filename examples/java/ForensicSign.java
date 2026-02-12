package com.provn.examples;

import com.provn.sdk.ProvnSDK;

/**
 * Provncloud Signing Example (Java)
 * 
 * This example demonstrates the standard workflow for a "Sovereign" claim:
 * 1. Create a secure digital identity (Local Keypair)
 * 2. Prepare a statement of truth (Claim)
 * 3. Sign the claim locally (Raw data remains local)
 * 4. Verify the integrity and authorship
 */
public class ForensicSign {
    public static void main(String[] args) {
        System.out.println("--- PROVNCLOUD SDK (Java) ---");

        try {
            // 1. SECURE IDENTITY: Generate a unique Ed25519 keypair
            ProvnSDK.KeyPair keypair = ProvnSDK.generateKeypair();
            System.out.println("[1] Identity Generated: ed25519:" + keypair.exportPublicKey());

            // 2. TRUTH STATEMENT: Define the data to be anchored.
            // For large files or privacy-sensitive data, we compute a local hash first.
            byte[] sensitiveData = "Internal Audit Memo: #1234 - High Priority Security Patch applied.".getBytes();
            String assetHash = ProvnSDK.computeHash(sensitiveData);
            System.out.println("[2] Prepared Claim (Asset Hash): \"" + assetHash + "\"");

            // Create claim with current timestamp
            ProvnSDK.Claim claim = ProvnSDK.createClaim(assetHash, "");
            System.out.println("    Timestamp: " + claim.timestamp);

            // 3. SECURE SIGNING: Sign the claim locally
            // This produces a SignedClaim object containing the original data,
            // the public key, and the cryptographic signature.
            ProvnSDK.SignedClaim signedClaim = ProvnSDK.signClaim(claim, keypair);
            System.out.println("[3] Local Signature: " + signedClaim.signature);

            // 4. INDEPENDENT VERIFICATION: Confirm the claim is authentic
            // This can be done by anybody with the public key,
            // without needing to contact the Provncloud API.
            boolean isValid = ProvnSDK.verifyClaim(signedClaim);

            if (isValid) {
                System.out.println("[4] Verification: SUCCESS (Sovereign Guarantee Intact)");
            } else {
                System.out.println("[4] Verification: FAILED (Data Tampering Detected)");
            }

        } catch (ProvnSDK.SDKException e) {
            System.err.println("Error: " + e.getMessage());
            e.printStackTrace();
        }
    }
}
