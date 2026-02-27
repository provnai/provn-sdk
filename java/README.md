# Provncloud SDK (Java)

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

**Official Java SDK for privacy-preserving digital signatures and data anchoring.**

Provncloud SDK allows you to cryptographically sign data and anchor it to blockchain networks like Arweave AO and Solana without revealing the raw content.

## 🚀 Installation

### Maven
For now, you can install the SDK locally using:
```bash
mvn install
```

Then add to your `pom.xml`:
```xml
<dependency>
    <groupId>com.provn</groupId>
    <artifactId>provn-sdk</artifactId>
    <version>0.3.0</version>
</dependency>
```

## 💻 Usage

```java
import com.provn.sdk.ProvnSDK;
import java.time.Instant;

public class Main {
    public static void main(String[] args) throws Exception {
        ProvnSDK sdk = new ProvnSDK();

        // 1. Generate a new Ed25519 keypair
        ProvnSDK.KeyPair keys = sdk.generateKeyPair();

        // 2. Create a claim
        long timestamp = Instant.now().getEpochSecond();
        ProvnSDK.Claim claim = sdk.createClaimWithTimestamp("Sensitive User Data", timestamp, "{\"context\":\"test\"}");

        // 3. Sign the claim
        ProvnSDK.SignedClaim signed = sdk.signClaim(claim, keys.privateKey);

        // 4. Verify (Offline)
        boolean isValid = sdk.verifyClaim(signed);
        System.out.println("Signature valid: " + isValid);
    }
}
```

## 🛠 Features

- **Security Focus**: Built on Bouncy Castle for robust Ed25519 implementation.
- **Schema Alignment**: Explicitly configured for `snake_case` JSON fields to ensure cross-language interoperability via JCS (RFC 8785).
- **Validation**: Strict enforcement of 2KB payload limits.

## 📚 Resources

- [**Monorepo & Examples**](https://github.com/provnai/provn-sdk)
- [**Protocol Spec**](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## ⚖️ License

MIT License. See [LICENSE](https://github.com/provnai/provn-sdk/blob/main/LICENSE) for details.
