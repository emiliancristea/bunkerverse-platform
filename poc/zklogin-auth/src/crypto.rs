use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use base64::{Engine as _, engine::general_purpose};
use ring::{
    digest::{self, SHA256},
    hmac,
    rand::{SecureRandom, SystemRandom},
    signature::{Ed25519KeyPair, KeyPair as RingKeyPair},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{oauth::OAuthProvider, UserIdentity, zkproof::ZkProof};

/// Cryptographic key pair for signing and verification
#[derive(Debug, Clone)]
pub struct KeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Signature validation result
#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureValidation {
    pub is_valid: bool,
    pub algorithm: String,
    pub key_id: String,
    pub timestamp: DateTime<Utc>,
}

/// Cryptographic engine for zkLogin authentication
pub struct CryptoEngine {
    signing_key: Ed25519KeyPair,
    hmac_key: hmac::Key,
    random: SystemRandom,
    strong_crypto: bool,
}

impl CryptoEngine {
    /// Create new cryptographic engine
    pub fn new(strong_crypto: bool) -> Result<Self> {
        let random = SystemRandom::new();
        
        // Generate Ed25519 signing key
        let signing_key_bytes = Ed25519KeyPair::generate_pkcs8(&random)
            .map_err(|e| anyhow!("Failed to generate signing key: {:?}", e))?;
        
        let signing_key = Ed25519KeyPair::from_pkcs8(signing_key_bytes.as_ref())
            .map_err(|e| anyhow!("Failed to create signing key: {:?}", e))?;
        
        // Generate HMAC key for token signing
        let mut hmac_key_bytes = [0u8; 32];
        random.fill(&mut hmac_key_bytes)
            .map_err(|e| anyhow!("Failed to generate HMAC key: {:?}", e))?;
        
        let hmac_key = hmac::Key::new(hmac::HMAC_SHA256, &hmac_key_bytes);
        
        println!("✅ Cryptographic engine initialized (strong crypto: {})", strong_crypto);
        
        Ok(Self {
            signing_key,
            hmac_key,
            random,
            strong_crypto,
        })
    }
    
    /// Hash a string using SHA-256
    pub fn hash_string(&self, input: &str) -> Result<String> {
        let digest = digest::digest(&SHA256, input.as_bytes());
        Ok(hex::encode(digest.as_ref()))
    }
    
    /// Hash a ZK proof for storage/verification
    pub fn hash_proof(&self, proof: &ZkProof) -> Result<String> {
        let proof_data = serde_json::to_string(proof)
            .map_err(|e| anyhow!("Failed to serialize proof: {}", e))?;
        self.hash_string(&proof_data)
    }
    
    /// Derive deterministic user ID from email and provider
    pub fn derive_user_id(&self, email: &str, provider: &OAuthProvider) -> Result<String> {
        let input = format!("{}:{:?}", email, provider);
        let hash = self.hash_string(&input)?;
        
        // Take first 16 chars for a shorter, more manageable user ID
        Ok(format!("user_{}", &hash[..16]))
    }
    
    /// Generate secure random nonce
    pub fn generate_nonce(&self, length: usize) -> Result<String> {
        let mut bytes = vec![0u8; length];
        self.random.fill(&mut bytes)
            .map_err(|e| anyhow!("Failed to generate nonce: {:?}", e))?;
        Ok(hex::encode(bytes))
    }
    
    /// Generate JWT access token
    pub fn generate_jwt_token(&self, identity: &UserIdentity, lifetime: Duration) -> Result<String> {
        if !self.strong_crypto {
            // Simple token for PoC when crypto disabled
            return Ok(format!("bunker_{}_{}", 
                             identity.user_id, 
                             Utc::now().timestamp()));
        }
        
        // Create JWT header
        let header = serde_json::json!({
            "alg": "EdDSA",
            "typ": "JWT",
            "kid": "bunkerverse_2025"
        });
        
        // Create JWT payload
        let exp = Utc::now() + chrono::Duration::from_std(lifetime)?;
        let payload = serde_json::json!({
            "iss": "bunkerverse.io",
            "sub": identity.user_id,
            "email": identity.email,
            "provider": format!("{:?}", identity.provider),
            "email_verified": identity.email_verified,
            "iat": Utc::now().timestamp(),
            "exp": exp.timestamp(),
            "nonce": identity.nonce_used,
            "proof_hash": identity.proof_hash
        });
        
        // Base64URL encode header and payload
        let header_b64 = general_purpose::URL_SAFE_NO_PAD.encode(
            serde_json::to_string(&header)?.as_bytes()
        );
        let payload_b64 = general_purpose::URL_SAFE_NO_PAD.encode(
            serde_json::to_string(&payload)?.as_bytes()
        );
        
        // Create signing input
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        
        // Sign with Ed25519
        let signature = self.signing_key.sign(signing_input.as_bytes());
        let signature_b64 = general_purpose::URL_SAFE_NO_PAD.encode(
            signature.as_ref()
        );
        
        Ok(format!("{}.{}", signing_input, signature_b64))
    }
    
    /// Validate JWT token signature
    pub fn validate_jwt_token(&self, token: &str) -> Result<SignatureValidation> {
        if !self.strong_crypto {
            // Simple validation for PoC
            return Ok(SignatureValidation {
                is_valid: token.starts_with("bunker_"),
                algorithm: "Simple".to_string(),
                key_id: "poc_key".to_string(),
                timestamp: Utc::now(),
            });
        }
        
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Ok(SignatureValidation {
                is_valid: false,
                algorithm: "EdDSA".to_string(),
                key_id: "bunkerverse_2025".to_string(),
                timestamp: Utc::now(),
            });
        }
        
        // Reconstruct signing input
        let signing_input = format!("{}.{}", parts[0], parts[1]);
        
        // Decode signature
        let signature_bytes = general_purpose::URL_SAFE_NO_PAD.decode(parts[2])
            .map_err(|e| anyhow!("Invalid signature encoding: {}", e))?;
        
        // Verify signature using public key
        let public_key_bytes = self.signing_key.public_key();
        let verification_result = ring::signature::UnparsedPublicKey::new(
            &ring::signature::ED25519,
            public_key_bytes.as_ref()
        ).verify(signing_input.as_bytes(), &signature_bytes);
        
        Ok(SignatureValidation {
            is_valid: verification_result.is_ok(),
            algorithm: "EdDSA".to_string(),
            key_id: "bunkerverse_2025".to_string(),
            timestamp: Utc::now(),
        })
    }
    
    /// Generate HMAC for data integrity
    pub fn generate_hmac(&self, data: &str) -> Result<String> {
        let signature = hmac::sign(&self.hmac_key, data.as_bytes());
        Ok(hex::encode(signature.as_ref()))
    }
    
    /// Verify HMAC signature
    pub fn verify_hmac(&self, data: &str, expected_hmac: &str) -> Result<bool> {
        let computed_hmac = self.generate_hmac(data)?;
        Ok(computed_hmac == expected_hmac)
    }
    
    /// Get public key for verification
    pub fn get_public_key(&self) -> Vec<u8> {
        self.signing_key.public_key().as_ref().to_vec()
    }
    
    /// Create key pair for external use
    pub fn create_key_pair(&self) -> Result<KeyPair> {
        let key_bytes = Ed25519KeyPair::generate_pkcs8(&self.random)
            .map_err(|e| anyhow!("Failed to generate key pair: {:?}", e))?;
        
        let key_pair = Ed25519KeyPair::from_pkcs8(key_bytes.as_ref())
            .map_err(|e| anyhow!("Failed to create key pair: {:?}", e))?;
        
        Ok(KeyPair {
            private_key: key_bytes.as_ref().to_vec(),
            public_key: key_pair.public_key().as_ref().to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oauth::OAuthProvider;
    
    #[test]
    fn test_crypto_engine_creation() {
        let engine = CryptoEngine::new(true).unwrap();
        assert!(engine.strong_crypto);
    }
    
    #[test]
    fn test_string_hashing() {
        let engine = CryptoEngine::new(false).unwrap();
        let hash1 = engine.hash_string("test").unwrap();
        let hash2 = engine.hash_string("test").unwrap();
        let hash3 = engine.hash_string("different").unwrap();
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA-256 hex = 64 chars
    }
    
    #[test]
    fn test_user_id_derivation() {
        let engine = CryptoEngine::new(false).unwrap();
        let user_id1 = engine.derive_user_id("test@example.com", &OAuthProvider::Google).unwrap();
        let user_id2 = engine.derive_user_id("test@example.com", &OAuthProvider::Google).unwrap();
        let user_id3 = engine.derive_user_id("test@example.com", &OAuthProvider::GitHub).unwrap();
        
        assert_eq!(user_id1, user_id2);
        assert_ne!(user_id1, user_id3);
        assert!(user_id1.starts_with("user_"));
    }
    
    #[test]
    fn test_nonce_generation() {
        let engine = CryptoEngine::new(false).unwrap();
        let nonce1 = engine.generate_nonce(16).unwrap();
        let nonce2 = engine.generate_nonce(16).unwrap();
        
        assert_ne!(nonce1, nonce2);
        assert_eq!(nonce1.len(), 32); // 16 bytes = 32 hex chars
    }
    
    #[test]
    fn test_hmac_generation_and_verification() {
        let engine = CryptoEngine::new(false).unwrap();
        let data = "test data";
        let hmac = engine.generate_hmac(data).unwrap();
        
        assert!(engine.verify_hmac(data, &hmac).unwrap());
        assert!(!engine.verify_hmac("different data", &hmac).unwrap());
    }
}