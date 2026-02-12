/**
 * Provncloud Signing Example (TypeScript)
 * 
 * This example demonstrates the standard workflow for a "Sovereign" claim:
 * 1. Create a secure digital identity (Local Keypair)
 * 2. Prepare a statement of truth (Claim)
 * 3. Sign the claim locally (Raw data remains local)
 * 4. Verify the integrity and authorship
 */

import { init, generateKeypair, computeHash, createClaimNow, signClaim, verifyClaim } from '@provn/sdk';

async function main() {
  console.log('--- PROVNCLOUD SDK (TypeScript) ---');

  // Initialize the SDK (loads WASM)
  await init();

  // 1. SECURE IDENTITY: Generate a unique Ed25519 keypair
  const keypair = generateKeypair();
  console.log(`[1] Identity Generated: ed25519:${keypair.publicKey}`);

  // 2. TRUTH STATEMENT: Define the data to be anchored.
  // For large files or privacy-sensitive data, we compute a local hash first.
  const sensitiveData = 'Internal Audit Memo: #1234 - High Priority Security Patch applied.';
  const assetHash = computeHash(sensitiveData);
  console.log(`[2] Prepared Claim (Asset Hash): "${assetHash}"`);

  // Create claim with current timestamp
  const claim = createClaimNow(assetHash);
  console.log(`    Timestamp: ${claim.timestamp}`);

  // 3. SECURE SIGNING: Sign the claim locally
  // This produces a SignedClaim object containing the original data,
  // the public key, and the cryptographic signature.
  const signedClaim = signClaim(claim, keypair);
  console.log(`[3] Local Signature: ${signedClaim.signature}`);

  // 4. INDEPENDENT VERIFICATION: Confirm the claim is authentic
  // This can be done by anybody with the public key,
  // without needing to contact the Provncloud API.
  const isValid = verifyClaim(signedClaim);

  if (isValid) {
    console.log('[4] Verification: SUCCESS (Sovereign Guarantee Intact)');
  } else {
    console.log('[4] Verification: FAILED (Data Tampering Detected)');
  }
}

main().catch(console.error);
