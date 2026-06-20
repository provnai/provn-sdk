#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Script to generate new cryptographic test vectors for edge cases.
Run from the repo root: python scripts/gen_test_vectors.py
Requires the provn_sdk Python package to be built and installed.
"""
import sys, os
import json

try:
    from provn_sdk import ProvnSDK
except ImportError:
    print("ERROR: provn_sdk not installed. Run `maturin develop --release` in python/ first.")
    sys.exit(1)

sdk = ProvnSDK()

# Fixed keys and constants
TIMESTAMP = 1704067200  # 2024-01-01 00:00:00 UTC

KEYS = {
    "claim_html_chars": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "claim_empty_metadata": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    "claim_unicode": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
}

def make_vector(name, description, data, metadata, timestamp):
    private_key = KEYS[name]
    claim = sdk.create_claim(data, timestamp, metadata)
    signed = sdk.sign_claim(claim, private_key)
    
    # Build canonical JSON manually to show expected format
    if metadata is not None:
        canonical = json.dumps({"data": data, "metadata": metadata, "timestamp": timestamp}, separators=(',', ':'), ensure_ascii=False)
    else:
        canonical = json.dumps({"data": data, "timestamp": timestamp}, separators=(',', ':'), ensure_ascii=False)

    return {
        "name": name,
        "description": description,
        "private_key": private_key,
        "public_key": signed['public_key'],
        "claim": signed['claim'],
        "canonical_json": canonical,
        "expected_signature": signed['signature']
    }

print("Generating edge-case test vectors...")

new_vectors = [
    make_vector(
        name="claim_html_chars",
        description="Claim with HTML characters in data field to catch JSON escaping bugs",
        data="a<b>&c",
        metadata=None,
        timestamp=TIMESTAMP
    ),
    make_vector(
        name="claim_empty_metadata",
        description="Claim with explicitly empty metadata string to catch omitempty bugs",
        data="test_empty_meta",
        metadata="",
        timestamp=TIMESTAMP
    ),
    make_vector(
        name="claim_unicode",
        description="Claim with non-ASCII unicode data to verify UTF-8 handling",
        data="日本語",
        metadata=None,
        timestamp=TIMESTAMP
    ),
]

# Path to test vectors JSON
vectors_path = os.path.join(os.path.dirname(__file__), '..', 'spec', 'test-vectors.json')

if os.path.exists(vectors_path):
    with open(vectors_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
else:
    data = {
        "generated_at": "2024-01-01T00:00:00Z",
        "sdk_version": "0.3.3",
        "spec_version": "1.0",
        "test_vectors": []
    }

# Update or append the vectors
existing_vectors = {v['name']: idx for idx, v in enumerate(data['test_vectors'])}

for v in new_vectors:
    if v['name'] in existing_vectors:
        idx = existing_vectors[v['name']]
        data['test_vectors'][idx] = v
        print(f"Updated existing vector: {v['name']}")
    else:
        data['test_vectors'].append(v)
        print(f"Added new vector: {v['name']}")

# Save back to file
with open(vectors_path, 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2, ensure_ascii=False)

print(f"Successfully wrote test vectors to {vectors_path}")
