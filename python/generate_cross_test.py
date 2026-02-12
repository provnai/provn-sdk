import provn_sdk
import json
import os

print(f"Generating cross-language test vector with SDK v{provn_sdk.get_version()}")

# 1. Generate Key
key = provn_sdk.generate_keypair()

# 2. Create Claim
# Using fixed timestamp for reproducibility in manual checks, though signature is deterministic anyway
claim = provn_sdk.create_claim("Hello Cross-Language World", 1700000000, "{\"env\": \"test\"}")

# 3. Sign
signed = provn_sdk.sign_claim(claim, key['private_key'])

# 4. Save to JSON
output_path = "../cross_lang_claim.json"
with open(output_path, "w") as f:
    json.dump(signed, f, indent=2)

print(f"Saved signed claim to {output_path}")
print(json.dumps(signed, indent=2))
