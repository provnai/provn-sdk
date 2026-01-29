use provn_sdk::{generate_keypair, sign_claim, verify_claim, Claim};

/// Provncloud Signing Example
///
/// This example demonstrates the standard workflow for a "Sovereign" claim:
/// 1. Create a secure digital identity (Local Keypair)
/// 2. Prepare a statement of truth (Claim)
/// 3. Sign the claim locally (Raw data remains local)
/// 4. Verify the integrity and authorship
fn main() {
    println!("--- PROVNCLOUD SDK ---");

    // 1. SECURE IDENTITY: Generate a unique Ed25519 keypair
    let signing_key = generate_keypair();
    let public_key = hex::encode(signing_key.verifying_key().as_bytes());
    println!("[1] Identity Generated: ed25519:{}", public_key);

    // 2. TRUTH STATEMENT: Define the data to be anchored.
    // For large files or privacy-sensitive data, we compute a local hash first.
    let sensitive_data = b"Internal Audit Memo: #1234 - High Priority Security Patch applied.";
    let asset_hash = provn_sdk::compute_hash(sensitive_data);

    let claim = Claim::new(asset_hash.clone());
    println!("[2] Prepared Claim (Asset Hash): \"{}\"", asset_hash);

    // 3. SECURE SIGNING: Sign the claim locally
    // This produces a SignedClaim object containing the original data,
    // the public key, and the cryptographic signature.
    let signed_claim = sign_claim(&claim, &signing_key).expect("Provncloud signing failed");
    println!("[3] Local Signature: {}", signed_claim.signature);

    // 4. INDEPENDENT VERIFICATION: Confirm the claim is authentic
    // This can be done by anybody with the public key,
    // without needing to contact the Provn API.
    let is_valid = verify_claim(&signed_claim).unwrap_or(false);

    if is_valid {
        println!("[4] Verification: SUCCESS (Sovereign Guarantee Intact)");
    } else {
        println!("[4] Verification: FAILED (Data Tampering Detected)");
    }
}
