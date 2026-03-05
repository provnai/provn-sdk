/**
 * Provncloud SDK for TypeScript
 * 
 * A privacy-preserving digital signature SDK for anchoring data to
 * Arweave AO and Solana blockchains.
 */

// Import WASM module
import {
  wasm_generate_keypair,
  wasm_compute_hash,
  wasm_sign_claim,
  wasm_verify_claim,
  wasm_create_claim_with_timestamp,
  wasm_version,
} from '../wasm/provn_sdk_wasm.js';

// Track initialization state
let initialized = false;

/**
 * Initialize the WASM module
 * 
 * Must be called before using any SDK functions
 */
export async function initSDK(): Promise<void> {
  // For Node.js WASM target, initialization is synchronous and automatic upon import.
  // This function is kept for API compatibility with browser targets.
  if (!initialized) {
    // await init(); 
    initialized = true;
  }
}

/**
 * Ensure SDK is initialized
 */
function ensureInitialized(): void {
  if (!initialized) {
    throw new Error('SDK not initialized. Call initSDK() first.');
  }
}

/**
 * KeyPair interface
 */
export interface KeyPair {
  private_key: string;  // Hex-encoded 32 bytes
  public_key: string;   // Hex-encoded 32 bytes
}

/**
 * Claim interface
 */
export interface Claim {
  data: string;
  timestamp: number;
  metadata?: string;
}

/**
 * SignedClaim interface
 */
export interface SignedClaim {
  claim: Claim;
  public_key: string;
  signature: string;
}

/**
 * Generate a new Ed25519 keypair
 * 
 * @returns KeyPair with hex-encoded keys
 */
export function generateKeypair(): KeyPair {
  ensureInitialized();
  const result = wasm_generate_keypair();
  const parsed = JSON.parse(result);
  return {
    private_key: parsed.private_key,
    public_key: parsed.public_key,
  };
}

/**
 * Compute SHA-256 hash of data
 * 
 * @param data - Data to hash
 * @returns Hex-encoded hash (64 characters)
 */
export function computeHash(data: Uint8Array | string): string {
  ensureInitialized();
  const bytes = typeof data === 'string' ? new TextEncoder().encode(data) : data;
  return wasm_compute_hash(bytes);
}

/**
 * Create a claim with explicit timestamp
 * 
 * @param data - The claim data (typically a hash)
 * @param timestamp - Unix timestamp in seconds
 * @param metadata - Optional metadata
 * @returns Claim object
 */
export function createClaim(
  data: string,
  timestamp: number,
  metadata?: string
): Claim {
  ensureInitialized();

  // Validate timestamp to prevent BigInt overflow or negative times
  if (timestamp < 0 || timestamp > Number.MAX_SAFE_INTEGER) {
    throw new Error("Invalid timestamp bounds.");
  }

  // Validate data is not empty
  if (!data || data.trim().length === 0) {
    throw new Error("Claim data cannot be empty.");
  }

  const jsonString = wasm_create_claim_with_timestamp(data, BigInt(timestamp), metadata || undefined);

  const encoder = new TextEncoder();
  if (encoder.encode(jsonString).length > 2048) {
    throw new Error("Error: Payload too large. Tip: For large datasets, hash the file locally and anchor the hash instead of the raw data.");
  }

  return JSON.parse(jsonString);
}

/**
 * Create a claim with current timestamp
 * 
 * @param data - The claim data (typically a hash)
 * @param metadata - Optional metadata
 * @returns Claim object
 */
export function createClaimNow(data: string, metadata?: string): Claim {
  const timestamp = Math.floor(Date.now() / 1000);
  return createClaim(data, timestamp, metadata);
}

/**
 * Sign a claim
 * 
 * @param claim - Claim to sign
 * @param keyPair - KeyPair containing private key
 * @returns SignedClaim
 */
export function signClaim(claim: Claim, keyPair: KeyPair): SignedClaim {
  ensureInitialized();
  const claimJson = JSON.stringify(claim);
  const signedJson = wasm_sign_claim(claimJson, keyPair.private_key);
  const parsed = JSON.parse(signedJson);
  return {
    claim: parsed.claim,
    public_key: parsed.public_key,
    signature: parsed.signature,
  };
}

/**
 * Error thrown when verification input is malformed (VULN-006).
 * Distinct from a clean cryptographic failure (which returns false).
 */
export class VerificationError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'VerificationError';
  }
}

/**
 * Verify a signed claim
 * 
 * @param signedClaim - SignedClaim to verify
 * @returns true if cryptographically valid, false if signature doesn't match
 * @throws VerificationError if the input is malformed (bad hex, missing fields, etc.)
 */
export function verifyClaim(signedClaim: SignedClaim): boolean {
  ensureInitialized();
  if (!signedClaim.claim || !signedClaim.public_key || !signedClaim.signature) {
    throw new VerificationError('Missing required fields: claim, public_key, signature');
  }
  const json = JSON.stringify({
    claim: signedClaim.claim,
    public_key: signedClaim.public_key,
    signature: signedClaim.signature,
  });
  try {
    return wasm_verify_claim(json);
  } catch (error) {
    // Re-throw as VerificationError so callers can distinguish input errors
    // from a successful-but-invalid signature check (which returns false).
    throw new VerificationError(`Verification failed: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Get SDK version
 * 
 * @returns Version string
 */
export function getVersion(): string {
  ensureInitialized();
  return wasm_version();
}

/**
 * ProvnSDK class - Alternative API
 * 
 * Provides a class-based interface wrapping the functional API
 */
export class ProvnSDK {
  private keyPair?: KeyPair;

  /**
   * Generate a new keypair and store it
   */
  generateKeypair(): KeyPair {
    this.keyPair = generateKeypair();
    return this.keyPair;
  }

  /**
   * Set keypair
   */
  setKeypair(keyPair: KeyPair): void {
    this.keyPair = keyPair;
  }

  /**
   * Get keypair
   */
  getKeypair(): KeyPair | undefined {
    return this.keyPair;
  }

  /**
   * Compute hash
   */
  computeHash(data: Uint8Array | string): string {
    return computeHash(data);
  }

  /**
   * Create claim
   */
  createClaim(data: string, timestamp: number, metadata?: string): Claim {
    return createClaim(data, timestamp, metadata);
  }

  /**
   * Create claim with current timestamp
   */
  createClaimNow(data: string, metadata?: string): Claim {
    return createClaimNow(data, metadata);
  }

  /**
   * Sign claim (requires keypair to be set)
   */
  signClaim(claim: Claim): SignedClaim {
    if (!this.keyPair) {
      throw new Error('No keypair set. Call generateKeypair() or setKeypair() first.');
    }
    return signClaim(claim, this.keyPair);
  }

  /**
   * Verify claim
   */
  verifyClaim(signedClaim: SignedClaim): boolean {
    return verifyClaim(signedClaim);
  }
}

// Re-export types
export { initSDK as init };
export default ProvnSDK;
