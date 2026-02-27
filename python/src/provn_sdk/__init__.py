"""
Provncloud SDK for Python

A privacy-preserving digital signature SDK for anchoring data to
Arweave AO and Solana blockchains.
"""

import json
from typing import Optional, Dict

# Import the PyO3 compiled native extension
from .provn_sdk import (
    generate_keypair as _generate_keypair,
    compute_hash as _compute_hash,
    create_claim as _create_claim,
    sign_claim as _sign_claim,
    verify_claim as _verify_claim,
    get_version as _get_version,
)

class ProvnSDK:
    """Provncloud SDK for Python"""

    def __init__(self):
        """Initialize the SDK"""
        pass

    def generate_keypair(self) -> Dict[str, str]:
        """Generate a new Ed25519 keypair"""
        return _generate_keypair()

    def compute_hash(self, data: bytes) -> str:
        """Compute SHA-256 hash of data"""
        return _compute_hash(data)

    def create_claim(
        self, data: str, timestamp: int, metadata: Optional[str] = None
    ) -> Dict:
        """Create a claim with explicit timestamp"""
        # Validate data
        if not data or len(data.strip()) == 0:
            raise ValueError("Claim data cannot be empty.")
            
        return _create_claim(data, timestamp, metadata)

    def sign_claim(self, claim: Dict, private_key_hex: str) -> Dict:
        """Sign a claim"""
        return _sign_claim(claim, private_key_hex)

    def verify_claim(self, signed_claim: Dict) -> bool:
        """Verify a signed claim"""
        try:
            return _verify_claim(signed_claim)
        except ValueError:
            # Re-raise standard value errors for structural mismatches
            return False

    def get_version(self) -> str:
        """Get SDK version"""
        return _get_version()
