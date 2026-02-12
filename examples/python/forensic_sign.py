#!/usr/bin/env python3
"""
Provncloud Signing Example (Python)

This example demonstrates the standard workflow for a "Sovereign" claim:
1. Create a secure digital identity (Local Keypair)
2. Prepare a statement of truth (Claim)
3. Sign the claim locally (Raw data remains local)
4. Verify the integrity and authorship
"""

import provn_sdk
import time
import json


def main():
    print("--- PROVNCLOUD SDK (Python) ---")
    print(f"SDK Version: {provn_sdk.get_version()}")

    # 1. SECURE IDENTITY: Generate a unique Ed25519 keypair
    # Returns a dict with 'private_key' and 'public_key' hex strings
    keypair = provn_sdk.generate_keypair()
    print(f"[1] Identity Generated: ed25519:{keypair['public_key']}")

    # 2. TRUTH STATEMENT: Define the data to be anchored.
    # For large files or privacy-sensitive data, we compute a local hash first.
    sensitive_data = (
        b"Internal Audit Memo: #1234 - High Priority Security Patch applied."
    )

    # Compute SHA-256 hash
    asset_hash = provn_sdk.compute_hash(sensitive_data)
    print(f'[2] Prepared Claim (Asset Hash): "{asset_hash}"')

    # Create claim with current timestamp
    # Returns a dict representing the claim
    claim = provn_sdk.create_claim(
        asset_hash, int(time.time()), json.dumps({"env": "prod"})
    )
    print(f"    Timestamp: {claim['timestamp']}")

    # 3. SECURE SIGNING: Sign the claim locally
    # This produces a SignedClaim object containing the original data,
    # the public key, and the cryptographic signature.
    signed_claim = provn_sdk.sign_claim(claim, keypair["private_key"])
    print(f"[3] Local Signature: {signed_claim['signature']}")

    # 4. INDEPENDENT VERIFICATION: Confirm the claim is authentic
    # This can be done by anybody with the public key,
    # without needing to contact the Provncloud API.
    is_valid = provn_sdk.verify_claim(signed_claim)

    if is_valid:
        print("[4] Verification: SUCCESS (Sovereign Guarantee Intact)")
    else:
        print("[4] Verification: FAILED (Data Tampering Detected)")


if __name__ == "__main__":
    main()
