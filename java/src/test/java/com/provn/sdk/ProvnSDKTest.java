package com.provn.sdk;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public class ProvnSDKTest {
    
    @Test
    public void testGenerateKeypair() {
        ProvnSDK.KeyPair kp = ProvnSDK.generateKeypair();
        
        assertNotNull(kp.privateKey);
        assertNotNull(kp.publicKey);
        assertEquals(32, kp.privateKey.length);
        assertEquals(32, kp.publicKey.length);
        
        String privHex = kp.exportPrivateKey();
        String pubHex = kp.exportPublicKey();
        
        assertEquals(64, privHex.length());
        assertEquals(64, pubHex.length());
    }
    
    @Test
    public void testComputeHash() {
        String hash = ProvnSDK.computeHash("hello".getBytes());
        
        assertEquals("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", hash);
        assertEquals(64, hash.length());
    }
    
    @Test
    public void testCreateClaim() throws ProvnSDK.SDKException {
        ProvnSDK.Claim claim = ProvnSDK.createClaim("test_data", "test_metadata");
        
        assertEquals("test_data", claim.data);
        assertEquals("test_metadata", claim.metadata);
        assertTrue(claim.timestamp > 0);
    }
    
    @Test
    public void testSignAndVerifyClaim() throws ProvnSDK.SDKException {
        ProvnSDK.KeyPair kp = ProvnSDK.generateKeypair();
        ProvnSDK.Claim claim = ProvnSDK.createClaimWithTimestamp("test_data", 1234567890L, null);
        
        ProvnSDK.SignedClaim signed = ProvnSDK.signClaim(claim, kp);
        
        assertNotNull(signed);
        assertEquals(kp.exportPublicKey(), signed.publicKey);
        assertEquals(128, signed.signature.length());
        
        boolean valid = ProvnSDK.verifyClaim(signed);
        assertTrue(valid);
    }
    
    @Test
    public void testVerifyTamperedClaim() throws ProvnSDK.SDKException {
        ProvnSDK.KeyPair kp = ProvnSDK.generateKeypair();
        ProvnSDK.Claim claim = ProvnSDK.createClaimWithTimestamp("original_data", 1234567890L, null);
        
        ProvnSDK.SignedClaim signed = ProvnSDK.signClaim(claim, kp);
        signed.claim.data = "tampered_data";
        
        boolean valid = ProvnSDK.verifyClaim(signed);
        assertFalse(valid);
    }
    
    @Test
    public void testMetadataSizeLimit() throws ProvnSDK.SDKException {
        ProvnSDK.KeyPair kp = ProvnSDK.generateKeypair();
        StringBuilder largeMetadata = new StringBuilder();
        for (int i = 0; i < 3000; i++) {
            largeMetadata.append("a");
        }
        
        ProvnSDK.Claim claim = ProvnSDK.createClaimWithTimestamp("data", 1234567890L, largeMetadata.toString());
        
        assertThrows(ProvnSDK.SDKException.class, () -> {
            ProvnSDK.signClaim(claim, kp);
        });
    }
    
    @Test
    public void testGetVersion() {
        String version = ProvnSDK.getVersion();
        assertNotNull(version);
        assertFalse(version.isEmpty());
    }
}
