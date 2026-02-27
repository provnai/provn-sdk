package com.provn.sdk;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.SerializationFeature;
import org.bouncycastle.crypto.params.Ed25519PrivateKeyParameters;
import org.bouncycastle.crypto.params.Ed25519PublicKeyParameters;
import org.bouncycastle.crypto.signers.Ed25519Signer;
import org.bouncycastle.util.encoders.Hex;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.time.Instant;

/**
 * Provncloud SDK for Java
 * 
 * A privacy-preserving digital signature SDK for anchoring data to
 * Arweave AO and Solana blockchains.
 */
public class ProvnSDK {
    
    private static final int MAX_PAYLOAD_SIZE = 2048;
    private static final ObjectMapper mapper = new ObjectMapper()
            .setSerializationInclusion(JsonInclude.Include.NON_NULL)
            .setPropertyNamingStrategy(PropertyNamingStrategies.SNAKE_CASE)
            .disable(SerializationFeature.INDENT_OUTPUT);
    
    /**
     * Claim representing a statement of truth
     */
    @JsonPropertyOrder({"data", "metadata", "timestamp"})
    public static class Claim {
        public String data;
        public String metadata;
        public long timestamp;
        
        public Claim() {}
        
        public Claim(String data, long timestamp, String metadata) {
            this.data = data;
            this.timestamp = timestamp;
            this.metadata = metadata;
        }
    }
    
    /**
     * SignedClaim wraps a Claim with cryptographic proof
     */
    public static class SignedClaim {
        @JsonProperty("claim")
        public Claim claim;
        @JsonProperty("public_key")
        public String publicKey;
        @JsonProperty("signature")
        public String signature;
        
        public SignedClaim() {}
        
        public SignedClaim(Claim claim, String publicKey, String signature) {
            this.claim = claim;
            this.publicKey = publicKey;
            this.signature = signature;
        }
    }
    
    /**
     * KeyPair for Ed25519
     */
    public static class KeyPair {
        @JsonProperty("private_key")
        public final byte[] privateKey;
        @JsonProperty("public_key")
        public final byte[] publicKey;
        
        public KeyPair(byte[] privateKey, byte[] publicKey) {
            this.privateKey = privateKey;
            this.publicKey = publicKey;
        }
        
        /**
         * Export the private key as hex.
         * WARNING: This seed must be heavily guarded. It grants full control over the identity. 
         */
        public String exportPrivateKey() {
            return Hex.toHexString(privateKey);
        }
        
        public String exportPublicKey() {
            return Hex.toHexString(publicKey);
        }
    }
    
    /**
     * Generate a new Ed25519 keypair
     */
    public static KeyPair generateKeypair() {
        SecureRandom random;
        try {
            random = SecureRandom.getInstanceStrong();
        } catch (NoSuchAlgorithmException e) {
            random = new SecureRandom(); // Fallback to default if strong algorithm not available
        }
        
        byte[] privateKeyBytes = new byte[32];
        random.nextBytes(privateKeyBytes);
        
        Ed25519PrivateKeyParameters privateKey = new Ed25519PrivateKeyParameters(privateKeyBytes, 0);
        Ed25519PublicKeyParameters publicKey = privateKey.generatePublicKey();
        
        return new KeyPair(privateKeyBytes, publicKey.getEncoded());
    }
    
    /**
     * Compute SHA-256 hash
     */
    public static String computeHash(byte[] data) {
        try {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] hash = digest.digest(data);
            return Hex.toHexString(hash);
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("SHA-256 not available", e);
        }
    }
    
    /**
     * Create a claim with current timestamp
     */
    public static Claim createClaim(String data, String metadata) {
        return new Claim(data, Instant.now().getEpochSecond(), metadata);
    }
    
    /**
     * Create a claim with explicit timestamp
     */
    public static Claim createClaimWithTimestamp(String data, long timestamp, String metadata) throws SDKException {
        if (data == null || data.trim().isEmpty()) {
            throw new SDKException("ValidationError", "Data field cannot be empty.");
        }
        if (timestamp < 1 || timestamp > 32503680000L) {
             throw new SDKException("ValidationError", "Timestamp out of bounds.");
        }
        return new Claim(data, timestamp, metadata);
    }
    
    /**
     * Serialize claim to canonical JSON bytes
     */
    public static byte[] toSignableBytes(Claim claim) throws SDKException {
        try {
            String json = mapper.writeValueAsString(claim);
            byte[] bytes = json.getBytes(StandardCharsets.UTF_8);
            
            if (bytes.length > MAX_PAYLOAD_SIZE) {
                throw new SDKException("ValidationError", 
                    "Error: Payload too large. Tip: For large datasets, hash the file locally and anchor the hash instead of the raw data.");
            }
            
            return bytes;
        } catch (JsonProcessingException e) {
            throw new SDKException("SerializationError", e.getMessage());
        }
    }
    
    /**
     * Sign a claim
     */
    public static SignedClaim signClaim(Claim claim, KeyPair keyPair) throws SDKException {
        byte[] message = toSignableBytes(claim);
        
        Ed25519PrivateKeyParameters privateKey = new Ed25519PrivateKeyParameters(keyPair.privateKey, 0);
        Ed25519Signer signer = new Ed25519Signer();
        signer.init(true, privateKey);
        signer.update(message, 0, message.length);
        byte[] signature = signer.generateSignature();
        
        return new SignedClaim(claim, Hex.toHexString(keyPair.publicKey), Hex.toHexString(signature));
    }
    
    /**
     * Verify a signed claim
     */
    public static boolean verifyClaim(SignedClaim signedClaim) throws SDKException {
        try {
            byte[] publicKeyBytes = Hex.decode(signedClaim.publicKey);
            byte[] signatureBytes = Hex.decode(signedClaim.signature);
            byte[] message = toSignableBytes(signedClaim.claim);
            
            Ed25519PublicKeyParameters publicKey = new Ed25519PublicKeyParameters(publicKeyBytes, 0);
            Ed25519Signer signer = new Ed25519Signer();
            signer.init(false, publicKey);
            signer.update(message, 0, message.length);
            
            return signer.verifySignature(signatureBytes);
        } catch (Exception e) {
            throw new SDKException("SignatureError", e.getMessage());
        }
    }
    
    /**
     * Get SDK version
     */
    public static String getVersion() {
        return "0.2.0";
    }
    
    /**
     * SDK Exception
     */
    public static class SDKException extends Exception {
        public final String type;
        
        public SDKException(String type, String message) {
            super(message);
            this.type = type;
        }
    }
}
