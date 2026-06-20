use provn_sdk::{verify_claim, SignedClaim};
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("../cross_lang_claim.json");
    if !path.exists() {
        eprintln!("Error: '../cross_lang_claim.json' not found!");
        eprintln!("Please run 'python generate_cross_test.py' in the python/ directory first to generate the test claim.");
        std::process::exit(1);
    }

    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    println!("Reading claim from: {:?}", canonical_path);

    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read cross_lang_claim.json: {}", e);
        std::process::exit(1);
    });
    let signed_claim: SignedClaim = serde_json::from_str(&content).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON into SignedClaim: {}", e);
        std::process::exit(1);
    });

    println!("Verifying claim...");
    println!("Data: {}", signed_claim.claim.data);
    println!("Timestamp: {}", signed_claim.claim.timestamp);

    let valid = verify_claim(&signed_claim).expect("Verification function returned error");

    if valid {
        println!("✅ Rust SDK successfully verified the Python-generated signature!");
    } else {
        eprintln!("❌ Signature verification FAILED!");
        std::process::exit(1);
    }
}
