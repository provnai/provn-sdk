import json
import pytest
import time
from provn_sdk import ProvnSDK

def test_generate_keypair():
    sdk = ProvnSDK()
    keys = sdk.generate_keypair()
    
    assert "private_key" in keys
    assert "public_key" in keys
    assert len(keys["private_key"]) == 64
    assert len(keys["public_key"]) == 64

def test_create_and_sign_claim():
    sdk = ProvnSDK()
    keys = sdk.generate_keypair()
    
    timestamp = int(time.time())
    claim = sdk.create_claim("Test Data", timestamp, "Test Meta")
    
    assert claim["data"] == "Test Data"
    assert claim["timestamp"] == timestamp
    assert claim["metadata"] == "Test Meta"
    
    signed = sdk.sign_claim(claim, keys["private_key"])
    
    assert "claim" in signed
    assert "public_key" in signed
    assert "signature" in signed
    assert signed["public_key"] == keys["public_key"]

def test_verify_valid_claim():
    sdk = ProvnSDK()
    keys = sdk.generate_keypair()
    
    claim = sdk.create_claim("Valid Claim Data", int(time.time()), None)
    signed = sdk.sign_claim(claim, keys["private_key"])
    
    assert sdk.verify_claim(signed) is True

def test_verify_invalid_signature():
    sdk1 = ProvnSDK()
    keys1 = sdk1.generate_keypair()
    
    sdk2 = ProvnSDK()
    keys2 = sdk2.generate_keypair()
    
    claim = sdk1.create_claim("Data", int(time.time()), None)
    signed = sdk1.sign_claim(claim, keys1["private_key"])
    
    # Tamper with the public key
    signed["public_key"] = keys2["public_key"]
    
    assert sdk1.verify_claim(signed) is False

def test_payload_size_limit():
    sdk = ProvnSDK()
    keys = sdk.generate_keypair()
    
    # create a massive payload of > 2KB
    large_data = "A" * 3000
    
    timestamp = int(time.time())
    
    # It should pass creation
    claim = sdk.create_claim(large_data, timestamp, None)
    
    # but fail during signing
    with pytest.raises(ValueError):
        sdk.sign_claim(claim, keys["private_key"])
        
def test_compute_hash():
    sdk = ProvnSDK()
    
    # We should get a valid SHA256 hex digest
    data = b"Hello World"
    hash_hex = sdk.compute_hash(data)
    
    assert len(hash_hex) == 64
    assert isinstance(hash_hex, str)
