// Mock the WASM module for tests
jest.mock('../wasm/provn_sdk_wasm.js', () => ({
  __esModule: true,
  default: jest.fn(() => Promise.resolve()),
  wasm_generate_keypair: jest.fn(() => JSON.stringify({
    private_key: 'a'.repeat(64),
    public_key: 'b'.repeat(64),
  })),
  wasm_compute_hash: jest.fn(() => '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824'),
  wasm_sign_claim: jest.fn((claimJson, privateKey) => {
    const claim = JSON.parse(claimJson);
    return JSON.stringify({
      claim,
      public_key: 'b'.repeat(64),
      signature: 'c'.repeat(128),
    });
  }),
  wasm_verify_claim: jest.fn(() => true),
  wasm_create_claim_with_timestamp: jest.fn((data, timestamp, metadata) => {
    return JSON.stringify({ data, timestamp: Number(timestamp), metadata });
  }),
  wasm_version: jest.fn(() => '0.2.0'),
}));
