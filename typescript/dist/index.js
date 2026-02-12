"use strict";
/**
 * Provncloud SDK for TypeScript
 *
 * A privacy-preserving digital signature SDK for anchoring data to
 * Arweave AO and Solana blockchains.
 */
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.ProvnSDK = void 0;
exports.initSDK = initSDK;
exports.init = initSDK;
exports.generateKeypair = generateKeypair;
exports.computeHash = computeHash;
exports.createClaim = createClaim;
exports.createClaimNow = createClaimNow;
exports.signClaim = signClaim;
exports.verifyClaim = verifyClaim;
exports.getVersion = getVersion;
// Import WASM module
const provn_sdk_wasm_js_1 = __importStar(require("../wasm/provn_sdk_wasm.js"));
// Track initialization state
let initialized = false;
/**
 * Initialize the WASM module
 *
 * Must be called before using any SDK functions
 */
async function initSDK() {
    if (!initialized) {
        await (0, provn_sdk_wasm_js_1.default)();
        initialized = true;
    }
}
/**
 * Ensure SDK is initialized
 */
function ensureInitialized() {
    if (!initialized) {
        throw new Error('SDK not initialized. Call initSDK() first.');
    }
}
/**
 * Generate a new Ed25519 keypair
 *
 * @returns KeyPair with hex-encoded keys
 */
function generateKeypair() {
    ensureInitialized();
    const result = (0, provn_sdk_wasm_js_1.wasm_generate_keypair)();
    const parsed = JSON.parse(result);
    return {
        privateKey: parsed.private_key,
        publicKey: parsed.public_key,
    };
}
/**
 * Compute SHA-256 hash of data
 *
 * @param data - Data to hash
 * @returns Hex-encoded hash (64 characters)
 */
function computeHash(data) {
    ensureInitialized();
    const bytes = typeof data === 'string' ? new TextEncoder().encode(data) : data;
    return (0, provn_sdk_wasm_js_1.wasm_compute_hash)(bytes);
}
/**
 * Create a claim with explicit timestamp
 *
 * @param data - The claim data (typically a hash)
 * @param timestamp - Unix timestamp in seconds
 * @param metadata - Optional metadata
 * @returns Claim object
 */
function createClaim(data, timestamp, metadata) {
    ensureInitialized();
    const json = (0, provn_sdk_wasm_js_1.wasm_create_claim_with_timestamp)(data, BigInt(timestamp), metadata || undefined);
    return JSON.parse(json);
}
/**
 * Create a claim with current timestamp
 *
 * @param data - The claim data (typically a hash)
 * @param metadata - Optional metadata
 * @returns Claim object
 */
function createClaimNow(data, metadata) {
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
function signClaim(claim, keyPair) {
    ensureInitialized();
    const claimJson = JSON.stringify(claim);
    const signedJson = (0, provn_sdk_wasm_js_1.wasm_sign_claim)(claimJson, keyPair.privateKey);
    const parsed = JSON.parse(signedJson);
    return {
        claim: parsed.claim,
        publicKey: parsed.public_key,
        signature: parsed.signature,
    };
}
/**
 * Verify a signed claim
 *
 * @param signedClaim - SignedClaim to verify
 * @returns true if valid, false otherwise
 */
function verifyClaim(signedClaim) {
    ensureInitialized();
    const json = JSON.stringify({
        claim: signedClaim.claim,
        public_key: signedClaim.publicKey,
        signature: signedClaim.signature,
    });
    return (0, provn_sdk_wasm_js_1.wasm_verify_claim)(json);
}
/**
 * Get SDK version
 *
 * @returns Version string
 */
function getVersion() {
    ensureInitialized();
    return (0, provn_sdk_wasm_js_1.wasm_version)();
}
/**
 * ProvnSDK class - Alternative API
 *
 * Provides a class-based interface wrapping the functional API
 */
class ProvnSDK {
    /**
     * Generate a new keypair and store it
     */
    generateKeypair() {
        this.keyPair = generateKeypair();
        return this.keyPair;
    }
    /**
     * Set keypair
     */
    setKeypair(keyPair) {
        this.keyPair = keyPair;
    }
    /**
     * Get keypair
     */
    getKeypair() {
        return this.keyPair;
    }
    /**
     * Compute hash
     */
    computeHash(data) {
        return computeHash(data);
    }
    /**
     * Create claim
     */
    createClaim(data, timestamp, metadata) {
        return createClaim(data, timestamp, metadata);
    }
    /**
     * Create claim with current timestamp
     */
    createClaimNow(data, metadata) {
        return createClaimNow(data, metadata);
    }
    /**
     * Sign claim (requires keypair to be set)
     */
    signClaim(claim) {
        if (!this.keyPair) {
            throw new Error('No keypair set. Call generateKeypair() or setKeypair() first.');
        }
        return signClaim(claim, this.keyPair);
    }
    /**
     * Verify claim
     */
    verifyClaim(signedClaim) {
        return verifyClaim(signedClaim);
    }
}
exports.ProvnSDK = ProvnSDK;
exports.default = ProvnSDK;
//# sourceMappingURL=index.js.map