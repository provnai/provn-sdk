/**
 * Provncloud SDK for TypeScript
 *
 * A privacy-preserving digital signature SDK for anchoring data to
 * Arweave AO and Solana blockchains.
 */
/**
 * Initialize the WASM module
 *
 * Must be called before using any SDK functions
 */
export declare function initSDK(): Promise<void>;
/**
 * KeyPair interface
 */
export interface KeyPair {
    privateKey: string;
    publicKey: string;
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
    publicKey: string;
    signature: string;
}
/**
 * Generate a new Ed25519 keypair
 *
 * @returns KeyPair with hex-encoded keys
 */
export declare function generateKeypair(): KeyPair;
/**
 * Compute SHA-256 hash of data
 *
 * @param data - Data to hash
 * @returns Hex-encoded hash (64 characters)
 */
export declare function computeHash(data: Uint8Array | string): string;
/**
 * Create a claim with explicit timestamp
 *
 * @param data - The claim data (typically a hash)
 * @param timestamp - Unix timestamp in seconds
 * @param metadata - Optional metadata
 * @returns Claim object
 */
export declare function createClaim(data: string, timestamp: number, metadata?: string): Claim;
/**
 * Create a claim with current timestamp
 *
 * @param data - The claim data (typically a hash)
 * @param metadata - Optional metadata
 * @returns Claim object
 */
export declare function createClaimNow(data: string, metadata?: string): Claim;
/**
 * Sign a claim
 *
 * @param claim - Claim to sign
 * @param keyPair - KeyPair containing private key
 * @returns SignedClaim
 */
export declare function signClaim(claim: Claim, keyPair: KeyPair): SignedClaim;
/**
 * Verify a signed claim
 *
 * @param signedClaim - SignedClaim to verify
 * @returns true if valid, false otherwise
 */
export declare function verifyClaim(signedClaim: SignedClaim): boolean;
/**
 * Get SDK version
 *
 * @returns Version string
 */
export declare function getVersion(): string;
/**
 * ProvnSDK class - Alternative API
 *
 * Provides a class-based interface wrapping the functional API
 */
export declare class ProvnSDK {
    private keyPair?;
    /**
     * Generate a new keypair and store it
     */
    generateKeypair(): KeyPair;
    /**
     * Set keypair
     */
    setKeypair(keyPair: KeyPair): void;
    /**
     * Get keypair
     */
    getKeypair(): KeyPair | undefined;
    /**
     * Compute hash
     */
    computeHash(data: Uint8Array | string): string;
    /**
     * Create claim
     */
    createClaim(data: string, timestamp: number, metadata?: string): Claim;
    /**
     * Create claim with current timestamp
     */
    createClaimNow(data: string, metadata?: string): Claim;
    /**
     * Sign claim (requires keypair to be set)
     */
    signClaim(claim: Claim): SignedClaim;
    /**
     * Verify claim
     */
    verifyClaim(signedClaim: SignedClaim): boolean;
}
export { initSDK as init };
export default ProvnSDK;
//# sourceMappingURL=index.d.ts.map