use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString}; 
use provn_sdk_core::{Claim, SignedClaim};
use provn_sdk_core::generate_keypair as rs_generate_keypair;
use provn_sdk_core::sign_claim as rs_sign_claim;
use provn_sdk_core::verify_claim as rs_verify_claim;
use provn_sdk_core::compute_hash as rs_compute_hash;
use ed25519_dalek::SigningKey;
use std::collections::HashMap;

/// Compute SHA-256 hash of data
#[pyfunction]
fn compute_hash(data: &[u8]) -> String {
    rs_compute_hash(data)
}

/// Generate a new keypair
/// Returns dict: {"private_key": hex, "public_key": hex}
#[pyfunction]
fn generate_keypair() -> HashMap<String, String> {
    let key = rs_generate_keypair();
    let mut map = HashMap::new();
    map.insert("private_key".to_string(), hex::encode(key.to_bytes()));
    map.insert("public_key".to_string(), hex::encode(key.verifying_key().as_bytes()));
    map
}

/// Create a claim
/// Returns dict representing the claim
#[pyfunction]
fn create_claim(data: String, timestamp: u64, metadata: Option<String>) -> HashMap<String, PyObject> {
    let claim = Claim::new_with_timestamp(data, timestamp);
    // Add metadata if present
    let mut final_claim = claim;
    final_claim.metadata = metadata;

    let mut map = HashMap::new();
    Python::with_gil(|py| {
        map.insert("data".to_string(), final_claim.data.to_object(py));
        map.insert("timestamp".to_string(), final_claim.timestamp.to_object(py));
        if let Some(meta) = final_claim.metadata {
            map.insert("metadata".to_string(), meta.to_object(py));
        }
    });
    map
}

/// Sign a claim
/// claim_dict: Dict from create_claim
/// private_key_hex: Hex string
#[pyfunction]
fn sign_claim(py: Python, claim_dict: &PyDict, private_key_hex: String) -> PyResult<HashMap<String, PyObject>> {
    // Reconstruct Claim from Dict
    let data_obj = claim_dict.get_item("data")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing data"))?;
    let data: String = data_obj.extract()?;
    
    let timestamp_obj = claim_dict.get_item("timestamp")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing timestamp"))?;
    let timestamp: u64 = timestamp_obj.extract()?;
    
    let metadata_obj = claim_dict.get_item("metadata")?;
    let metadata: Option<String> = if let Some(meta) = metadata_obj {
        if meta.is_none() {
            None
        } else {
            Some(meta.extract()?)
        }
    } else {
        None
    };

    let claim = Claim {
        data,
        timestamp,
        metadata,
    };

    // Decode Key
    let key_bytes = hex::decode(private_key_hex).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid key hex: {}", e)))?;
    let key_array: [u8; 32] = key_bytes.try_into().map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid key length"))?;
    let signing_key = SigningKey::from_bytes(&key_array);

    // Sign
    let signed = rs_sign_claim(&claim, &signing_key).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Signing failed: {}", e)))?;

    // Return Dict
    let mut result = HashMap::new();
    
    // Convert Claim back to dict for return
    let mut claim_map = HashMap::new();
    claim_map.insert("data".to_string(), signed.claim.data.to_object(py));
    claim_map.insert("timestamp".to_string(), signed.claim.timestamp.to_object(py));
    if let Some(meta) = signed.claim.metadata {
        claim_map.insert("metadata".to_string(), meta.to_object(py));
    }

    result.insert("claim".to_string(), claim_map.to_object(py));
    result.insert("public_key".to_string(), signed.public_key.to_object(py));
    result.insert("signature".to_string(), signed.signature.to_object(py));

    Ok(result)
}

/// Verify a signed claim
#[pyfunction]
fn verify_claim(py: Python, signed_claim_dict: &PyDict) -> PyResult<bool> {
    // Extract Public Key & Signature
    let pk_hex: String = signed_claim_dict.get_item("public_key")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing public_key"))?.extract()?;
    let sig_hex: String = signed_claim_dict.get_item("signature")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing signature"))?.extract()?;
    
    // Extract Claim
    let claim_item = signed_claim_dict.get_item("claim")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing claim"))?;
    let claim_dict: &PyDict = claim_item.downcast()?;
    
    let data: String = claim_dict.get_item("data")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing claim data"))?.extract()?;
    let timestamp: u64 = claim_dict.get_item("timestamp")?.ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Missing claim timestamp"))?.extract()?;
    
    let metadata_item = claim_dict.get_item("metadata")?;
    let metadata: Option<String> = if let Some(meta) = metadata_item {
        if meta.is_none() {
            None
        } else {
            Some(meta.extract()?)
        }
    } else {
        None
    };

    let claim = Claim {
        data,
        timestamp,
        metadata,
    };

    let signed_claim = SignedClaim {
        claim,
        public_key: pk_hex,
        signature: sig_hex,
    };

    match rs_verify_claim(&signed_claim) {
        Ok(valid) => Ok(valid),
        Err(_) => Ok(false),
    }
}

/// Get SDK version
#[pyfunction]
fn get_version() -> String {
    "0.2.0".to_string()
}

#[pymodule]
fn provn_sdk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_hash, m)?)?;
    m.add_function(wrap_pyfunction!(generate_keypair, m)?)?;
    m.add_function(wrap_pyfunction!(create_claim, m)?)?;
    m.add_function(wrap_pyfunction!(sign_claim, m)?)?;
    m.add_function(wrap_pyfunction!(verify_claim, m)?)?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    Ok(())
}
