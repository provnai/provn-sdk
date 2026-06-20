# Provncloud SDK (Java)

[![License: MIT](https://img.shields.io/badge/License-MIT-emerald.svg)](https://opensource.org/licenses/MIT)

Java implementation of the Provncloud claim signing format using Jackson and Bouncy Castle.

## Installation

Build locally:

```bash
mvn clean install
```

Dependency:

```xml
<dependency>
    <groupId>com.provn</groupId>
    <artifactId>provn-sdk</artifactId>
    <version>0.3.3</version>
</dependency>
```

## Usage

```java
import com.provn.sdk.ProvnSDK;

public class Main {
    public static void main(String[] args) throws Exception {
        ProvnSDK.KeyPair keys = ProvnSDK.generateKeypair();
        ProvnSDK.Claim claim =
            ProvnSDK.createClaimWithTimestamp("Sensitive User Data", 1704067200L, null);
        ProvnSDK.SignedClaim signed = ProvnSDK.signClaim(claim, keys);

        System.out.println(ProvnSDK.verifyClaim(signed));
    }
}
```

## Notes

- Jackson is configured for `snake_case`, ordered claim properties, and strict unknown-field rejection.
- `Claim` and `SignedClaim` are immutable value objects.
- Canonical payload size is limited to 2 KB.

## Resources

- [Monorepo](https://github.com/provnai/provn-sdk)
- [Protocol Spec](https://github.com/provnai/provn-sdk/blob/main/spec/SPEC.md)

## License

MIT
