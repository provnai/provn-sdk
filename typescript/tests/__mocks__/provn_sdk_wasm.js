const keyRegistry = new Map();
let keyCounter = 0;

function makeHex(value) {
  return value.toString(16).padStart(64, "0");
}

module.exports = {
  __esModule: true,
  default: jest.fn(() => Promise.resolve()),
  wasm_generate_keypair: jest.fn(() => {
    keyCounter += 1;
    const private_key = makeHex(keyCounter);
    const public_key = makeHex(keyCounter + 4096);
    keyRegistry.set(private_key, public_key);
    return JSON.stringify({ private_key, public_key });
  }),
  wasm_compute_hash: jest.fn(() => '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824'),
  wasm_sign_claim: jest.fn((claimJson, privateKey) => {
    const claim = JSON.parse(claimJson);
    const public_key = keyRegistry.get(privateKey) || makeHex(4096);
    return JSON.stringify({
      claim,
      public_key,
      signature: public_key.repeat(2),
    });
  }),
  wasm_verify_claim: jest.fn((signedClaimJson) => {
    const signed = JSON.parse(signedClaimJson);
    if (signed.public_key === 'malformed') {
      throw new Error('Malformed payload');
    }
    if (
      signed.claim?.data === 'tampered_data' ||
      signed.signature !== signed.public_key.repeat(2)
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
