use provn_sdk::{verify_claim, SignedClaim};
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("../cross_lang_claim.json");
    println!("Reading claim from: {:?}", path.canonicalize());

    let content = fs::read_to_string(path).expect("Failed to read cross_lang_claim.json");
    let signed_claim: SignedClaim =
        serde_json::from_str(&content).expect("Failed to parse JSON into SignedClaim");

    println!("Verifying claim...");
    println!("Data: {}", signed_claim.claim.data);
    println!("Timestamp: {}", signed_claim.claim.timestamp);

    let valid = verify_claim(&signed_claim).expect("Verification function returned error");

    if valid {
        println!("✅ Rust SDK successfully verified the Python-generated signature!");
    } else {
        panic!("❌ Signature verification FAILED!");
    }
}
