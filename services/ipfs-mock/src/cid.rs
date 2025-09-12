use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Cid {
    pub hash: String,
    pub version: u8,
}

impl Cid {
    pub fn new(content: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash_bytes = hasher.finalize();

        // Convert to base58 (simplified)
        let hash = bs58::encode(&hash_bytes).into_string();

        Self {
            hash: format!("Qm{}", &hash[..44.min(hash.len())]),
            version: 0,
        }
    }

    pub fn from_string(cid_str: &str) -> Result<Self, CidError> {
        if cid_str.is_empty() {
            return Err(CidError::InvalidFormat("Empty CID".to_string()));
        }

        // Basic validation for CIDv0 (starts with Qm and has correct length)
        if cid_str.starts_with("Qm") && cid_str.len() >= 44 {
            Ok(Self {
                hash: cid_str.to_string(),
                version: 0,
            })
        }
        // Basic validation for CIDv1 (starts with b and uses base32)
        else if cid_str.starts_with("b") && cid_str.len() > 10 {
            Ok(Self {
                hash: cid_str.to_string(),
                version: 1,
            })
        }
        // Accept any hash-like string for mock purposes
        else if cid_str.len() >= 10 {
            Ok(Self {
                hash: cid_str.to_string(),
                version: 0,
            })
        } else {
            Err(CidError::InvalidFormat(format!(
                "Invalid CID format: {}",
                cid_str
            )))
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.hash.is_empty() && self.hash.len() >= 10
    }

    pub fn generate_mock(seed: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        hasher.update(b"mock-ipfs-content");
        let hash_bytes = hasher.finalize();

        let hash = bs58::encode(&hash_bytes).into_string();

        Self {
            hash: format!("Qm{}", &hash[..44.min(hash.len())]),
            version: 0,
        }
    }
}

impl fmt::Display for Cid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hash)
    }
}

#[derive(Debug, Clone)]
pub enum CidError {
    InvalidFormat(String),
    UnsupportedVersion(u8),
}

impl fmt::Display for CidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CidError::InvalidFormat(msg) => write!(f, "Invalid CID format: {}", msg),
            CidError::UnsupportedVersion(version) => {
                write!(f, "Unsupported CID version: {}", version)
            }
        }
    }
}

impl std::error::Error for CidError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_creation() {
        let content = b"Hello, IPFS!";
        let cid = Cid::new(content);
        assert!(cid.is_valid());
        assert_eq!(cid.version, 0);
    }

    #[test]
    fn test_cid_from_string() {
        let valid_cid = "QmT78zSuBmuS4z925WZfrqQ1qHaJ56DQaTfyMUF7F8ff5o";
        let cid = Cid::from_string(valid_cid).unwrap();
        assert_eq!(cid.hash, valid_cid);
        assert_eq!(cid.version, 0);
    }

    #[test]
    fn test_invalid_cid() {
        let invalid_cid = "invalid";
        let result = Cid::from_string(invalid_cid);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_cid_generation() {
        let seed = "test-seed";
        let cid1 = Cid::generate_mock(seed);
        let cid2 = Cid::generate_mock(seed);

        // Same seed should produce same CID
        assert_eq!(cid1.hash, cid2.hash);

        let cid3 = Cid::generate_mock("different-seed");
        assert_ne!(cid1.hash, cid3.hash);
    }
}
