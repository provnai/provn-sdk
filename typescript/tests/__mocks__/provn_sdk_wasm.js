module.exports = {
  __esModule: true,
  default: jest.fn(() => Promise.resolve()),
  wasm_generate_keypair: jest.fn(() => JSON.stringify({
    private_key: 'a'.repeat(64),
    public_key: 'b'.repeat(64),
  })),
  wasm_compute_hash: jest.fn(() => '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824'),
  wasm_sign_claim: jest.fn((claimJson) => {
    const claim = JSON.parse(claimJson);
    return JSON.stringify({
      claim,
      public_key: 'b'.repeat(64),
      signature: 'c'.repeat(128),
    });
  }),
  wasm_verify_claim: jest.fn((signedClaimJson) => {
    const signed = JSON.parse(signedClaimJson);
    if (signed.public_key === 'malformed') {
      throw new Error('Malformed payload');
    }
    if (
      signed.claim?.data === 'tampered_data' ||
      signed.public_key !== 'b'.repeat(64) ||
      signed.signature !== 'c'.repeat(128)
    ) {
      throw new Error('Invalid signature');
    }
    return true;
  }),
  wasm_create_claim_with_timestamp: jest.fn((data, timestamp, metadata) =>
    JSON.stringify({ data, timestamp: Number(timestamp), metadata })
  ),
  wasm_version: jest.fn(() => '0.3.3'),
};
