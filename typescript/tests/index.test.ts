/**
 * Provncloud SDK TypeScript Tests
 */

jest.mock('../wasm/provn_sdk_wasm.js', () => require('./__mocks__/provn_sdk_wasm.js'));

import {
  generateKeypair,
  computeHash,
  createClaim,
  signClaim,
  verifyClaim,
  createClaimNow,
  getVersion,
  ProvnSDK,
  VerificationError,
} from '../src/index';
import { wasm_verify_claim, wasm_version } from '../wasm/provn_sdk_wasm.js';

describe('ProvnSDK', () => {
  beforeEach(() => {
    (wasm_verify_claim as jest.Mock).mockReset();
    (wasm_verify_claim as jest.Mock).mockImplementation((signedClaimJson: string) => {
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
    });
    (wasm_version as jest.Mock).mockReset();
    (wasm_version as jest.Mock).mockImplementation(() => '0.3.3');
  });

  describe('generateKeypair', () => {
    it('should generate a valid keypair', () => {
      const keypair = generateKeypair();

      expect(keypair).toHaveProperty('private_key');
      expect(keypair).toHaveProperty('public_key');
      expect(keypair.private_key).toHaveLength(64);
      expect(keypair.public_key).toHaveLength(64);
      expect(keypair.private_key).toMatch(/^[0-9a-f]+$/);
      expect(keypair.public_key).toMatch(/^[0-9a-f]+$/);
    });

    it('should generate unique keypairs', () => {
      const keypair1 = generateKeypair();
      const keypair2 = generateKeypair();

      expect(keypair1.private_key).not.toBe(keypair2.private_key);
      expect(keypair1.public_key).not.toBe(keypair2.public_key);
    });
  });

  describe('computeHash', () => {
    it('should compute SHA-256 hash of string', () => {
      const hash = computeHash('hello');

      expect(hash).toHaveLength(64);
      expect(hash).toMatch(/^[0-9a-f]+$/);
      expect(hash).toBe('2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824');
    });

    it('should compute SHA-256 hash of bytes', () => {
      const bytes = new TextEncoder().encode('hello');
      const hash = computeHash(bytes);

      expect(hash).toHaveLength(64);
      expect(hash).toBe('2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824');
    });

    it('should return consistent hashes for same input', () => {
      const hash1 = computeHash('test');
      const hash2 = computeHash('test');

      expect(hash1).toBe(hash2);
    });
  });

  describe('createClaim', () => {
    it('should create a claim with given timestamp', () => {
      const claim = createClaim('data', 1234567890);

      expect(claim).toHaveProperty('data', 'data');
      expect(claim).toHaveProperty('timestamp', 1234567890);
      expect(claim.metadata).toBeUndefined();
    });

    it('should create a claim with metadata', () => {
      const claim = createClaim('data', 1234567890, 'metadata');

      expect(claim).toHaveProperty('data', 'data');
      expect(claim).toHaveProperty('timestamp', 1234567890);
      expect(claim).toHaveProperty('metadata', 'metadata');
    });
  });

  describe('createClaimNow', () => {
    it('should create a claim with current timestamp', () => {
      const before = Math.floor(Date.now() / 1000);
      const claim = createClaimNow('data');
      const after = Math.floor(Date.now() / 1000);

      expect(claim).toHaveProperty('data', 'data');
      expect(claim.timestamp).toBeGreaterThanOrEqual(before);
      expect(claim.timestamp).toBeLessThanOrEqual(after);
    });
  });

  describe('signClaim and verifyClaim', () => {
    it('should sign and verify a claim', () => {
      const keypair = generateKeypair();
      const claim = createClaim('test_data', 1234567890);

      const signed = signClaim(claim, keypair);

      expect(signed).toHaveProperty('claim');
      expect(signed).toHaveProperty('public_key');
      expect(signed).toHaveProperty('signature');
      expect(signed.claim).toEqual(claim);
      expect(signed.public_key).toBe(keypair.public_key);
      expect(signed.signature).toHaveLength(128);

      const isValid = verifyClaim(signed);
      expect(isValid).toBe(true);
    });

    it('should reject tampered claim', () => {
      const keypair = generateKeypair();
      const claim = createClaim('original_data', 1234567890);

      const signed = signClaim(claim, keypair);
      signed.claim.data = 'tampered_data';

      const isValid = verifyClaim(signed);
      expect(isValid).toBe(false);
    });

    it('should reject tampered signature', () => {
      const keypair = generateKeypair();
      const claim = createClaim('data', 1234567890);

      const signed = signClaim(claim, keypair);
      signed.signature = signed.signature.slice(0, -1) + 'a';

      const isValid = verifyClaim(signed);
      expect(isValid).toBe(false);
    });

    it('should reject wrong public key', () => {
      const keypair1 = generateKeypair();
      const keypair2 = generateKeypair();
      const claim = createClaim('data', 1234567890);

      const signed = signClaim(claim, keypair1);
      signed.public_key = keypair2.public_key;

      const isValid = verifyClaim(signed);
      expect(isValid).toBe(false);
    });

    it('should throw VerificationError for malformed input before WASM', () => {
      expect(() => verifyClaim({
        claim: undefined as never,
        public_key: 'a'.repeat(64),
        signature: 'b'.repeat(128),
      })).toThrow(VerificationError);
    });

    it('should throw VerificationError for malformed WASM errors', () => {
      const keypair = generateKeypair();
      const claim = createClaim('data', 1234567890);
      const signed = signClaim(claim, keypair);
      signed.public_key = 'malformed';

      expect(() => verifyClaim(signed)).toThrow(VerificationError);
    });
  });

  describe('getVersion', () => {
    it('should return version string', () => {
      const version = getVersion();

      expect(typeof version).toBe('string');
      expect(version).toBe('0.3.3');
    });
  });

  describe('ProvnSDK class', () => {
    it('should generate keypair', () => {
      const sdk = new ProvnSDK();
      const keypair = sdk.generateKeypair();

      expect(keypair).toHaveProperty('private_key');
      expect(keypair).toHaveProperty('public_key');
      expect(sdk.getKeypair()).toEqual(keypair);
    });

    it('should set keypair', () => {
      const sdk = new ProvnSDK();
      const keypair = generateKeypair();

      sdk.setKeypair(keypair);
      expect(sdk.getKeypair()).toEqual(keypair);
    });

    it('should sign claim with stored keypair', () => {
      const sdk = new ProvnSDK();
      const keypair = sdk.generateKeypair();
      const claim = createClaimNow('data');

      const signed = sdk.signClaim(claim);

      expect(signed.public_key).toBe(keypair.public_key);
      expect(verifyClaim(signed)).toBe(true);
    });

    it('should throw when signing without keypair', () => {
      const sdk = new ProvnSDK();
      const claim = createClaimNow('data');

      expect(() => sdk.signClaim(claim)).toThrow('No keypair set');
    });

    it('should verify claim', () => {
      const sdk = new ProvnSDK();
      const keypair = generateKeypair();
      const claim = createClaimNow('data');
      const signed = signClaim(claim, keypair);

      expect(sdk.verifyClaim(signed)).toBe(true);
    });
  });
});
