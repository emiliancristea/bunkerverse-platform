use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{anyhow, Context, Result};
use pbkdf2::{password_hash::{PasswordHasher, SaltString}, Pbkdf2};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
    pub iterations: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CipherType {
    Aes256Gcm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureDataBlob {
    pub encrypted_data: Vec<u8>,
    pub cipher_type: CipherType,
    pub metadata: EncryptionMetadata,
}

pub struct RedbEncryption {
    cipher: Aes256Gcm,
    key_info: KeyInfo,
}

impl RedbEncryption {
    pub fn new(password: &str) -> Result<Self> {
        let salt = Self::generate_salt()?;
        let key = Self::derive_key_from_password(password, &salt, PBKDF2_ITERATIONS)?;
        let cipher = Aes256Gcm::new(&key);
        
        let key_info = KeyInfo {
            salt,
            iterations: PBKDF2_ITERATIONS,
            algorithm: "AES-256-GCM".to_string(),
        };
        
        Ok(Self { cipher, key_info })
    }
    
    pub fn from_existing_key_info(password: &str, key_info: KeyInfo) -> Result<Self> {
        let key = Self::derive_key_from_password(password, &key_info.salt, key_info.iterations)?;
        let cipher = Aes256Gcm::new(&key);
        
        Ok(Self { cipher, key_info })
    }
    
    pub fn encrypt_data(&self, plaintext: &[u8]) -> Result<SecureDataBlob> {
        let nonce_bytes = Self::generate_nonce()?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;
        
        let metadata = EncryptionMetadata {
            nonce: nonce_bytes,
            salt: self.key_info.salt.clone(),
            iterations: self.key_info.iterations,
            created_at: chrono::Utc::now(),
        };
        
        Ok(SecureDataBlob {
            encrypted_data: ciphertext,
            cipher_type: CipherType::Aes256Gcm,
            metadata,
        })
    }
    
    pub fn decrypt_data(&self, secure_blob: &SecureDataBlob) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(&secure_blob.metadata.nonce);
        
        let plaintext = self.cipher
            .decrypt(nonce, secure_blob.encrypted_data.as_ref())
            .map_err(|e| anyhow!("Decryption failed: {}", e))?;
        
        Ok(plaintext)
    }
    
    pub fn get_key_info(&self) -> &KeyInfo {
        &self.key_info
    }
    
    fn generate_salt() -> Result<Vec<u8>> {
        let mut salt = vec![0u8; SALT_LENGTH];
        let mut rng = ChaCha20Rng::from_entropy();
        rng.fill_bytes(&mut salt);
        Ok(salt)
    }
    
    fn generate_nonce() -> Result<Vec<u8>> {
        let mut nonce = vec![0u8; NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce);
        Ok(nonce)
    }
    
    fn derive_key_from_password(password: &str, salt: &[u8], iterations: u32) -> Result<Key<Aes256Gcm>> {
        let salt_string = SaltString::from_b64(&base64::encode(salt))
            .map_err(|e| anyhow!("Invalid salt format: {}", e))?;
        
        let password_hash = Pbkdf2
            .hash_password_customized(
                password.as_bytes(),
                None,
                None,
                pbkdf2::Params {
                    rounds: iterations,
                    output_length: 32,
                },
                &salt_string,
            )
            .map_err(|e| anyhow!("PBKDF2 derivation failed: {}", e))?;
        
        let key_bytes = password_hash.hash.unwrap().as_bytes();
        if key_bytes.len() != 32 {
            return Err(anyhow!("Derived key has wrong length: {}", key_bytes.len()));
        }
        
        Ok(*Key::<Aes256Gcm>::from_slice(key_bytes))
    }
}

pub struct SecureRedbStorage {
    encryption: RedbEncryption,
    key_store: HashMap<String, KeyInfo>,
}

impl SecureRedbStorage {
    pub fn new(master_password: &str) -> Result<Self> {
        let encryption = RedbEncryption::new(master_password)?;
        let key_store = HashMap::new();
        
        Ok(Self {
            encryption,
            key_store,
        })
    }
    
    pub fn encrypt_for_storage(&self, data: &impl Serialize) -> Result<Vec<u8>> {
        let json_data = serde_json::to_vec(data)
            .context("Failed to serialize data for encryption")?;
        
        let secure_blob = self.encryption.encrypt_data(&json_data)?;
        
        serde_json::to_vec(&secure_blob)
            .context("Failed to serialize encrypted blob")
    }
    
    pub fn decrypt_from_storage<T: for<'de> Deserialize<'de>>(&self, encrypted_data: &[u8]) -> Result<T> {
        let secure_blob: SecureDataBlob = serde_json::from_slice(encrypted_data)
            .context("Failed to deserialize encrypted blob")?;
        
        let decrypted_data = self.encryption.decrypt_data(&secure_blob)?;
        
        serde_json::from_slice(&decrypted_data)
            .context("Failed to deserialize decrypted data")
    }
    
    pub fn rotate_encryption_key(&mut self, old_password: &str, new_password: &str) -> Result<()> {
        let new_encryption = RedbEncryption::new(new_password)?;
        
        self.key_store.insert(
            "previous_key".to_string(),
            self.encryption.get_key_info().clone(),
        );
        
        self.encryption = new_encryption;
        
        Ok(())
    }
    
    pub fn benchmark_encryption_performance(&self, data_sizes: &[usize]) -> Result<EncryptionBenchmark> {
        let mut results = EncryptionBenchmark::default();
        
        for &size in data_sizes {
            let test_data = vec![0u8; size];
            
            let encrypt_start = std::time::Instant::now();
            let encrypted = self.encryption.encrypt_data(&test_data)?;
            let encrypt_time = encrypt_start.elapsed();
            
            let decrypt_start = std::time::Instant::now();
            let _decrypted = self.encryption.decrypt_data(&encrypted)?;
            let decrypt_time = decrypt_start.elapsed();
            
            results.size_results.push(SizeBenchmark {
                data_size: size,
                encrypt_time,
                decrypt_time,
                encrypted_size: encrypted.encrypted_data.len(),
            });
        }
        
        Ok(results)
    }
}

#[derive(Debug, Default)]
pub struct EncryptionBenchmark {
    pub size_results: Vec<SizeBenchmark>,
}

#[derive(Debug)]
pub struct SizeBenchmark {
    pub data_size: usize,
    pub encrypt_time: std::time::Duration,
    pub decrypt_time: std::time::Duration,
    pub encrypted_size: usize,
}

impl EncryptionBenchmark {
    pub fn print_summary(&self) {
        println!("\n🔐 AES-256-GCM ENCRYPTION BENCHMARK");
        println!("===================================");
        
        for result in &self.size_results {
            let encrypt_speed = result.data_size as f64 / result.encrypt_time.as_secs_f64() / 1_048_576.0;
            let decrypt_speed = result.data_size as f64 / result.decrypt_time.as_secs_f64() / 1_048_576.0;
            let overhead = ((result.encrypted_size as f64 / result.data_size as f64) - 1.0) * 100.0;
            
            println!("📊 Data Size: {} bytes", result.data_size);
            println!("  • Encrypt: {:.2} MB/s ({:?})", encrypt_speed, result.encrypt_time);
            println!("  • Decrypt: {:.2} MB/s ({:?})", decrypt_speed, result.decrypt_time);
            println!("  • Overhead: {:.1}% ({} bytes)", overhead, result.encrypted_size - result.data_size);
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_roundtrip() {
        let encryption = RedbEncryption::new("test_password").unwrap();
        let original_data = b"Hello, Bunkerverse!";
        
        let encrypted = encryption.encrypt_data(original_data).unwrap();
        let decrypted = encryption.decrypt_data(&encrypted).unwrap();
        
        assert_eq!(original_data, &decrypted[..]);
    }
    
    #[test]
    fn test_secure_storage() {
        let storage = SecureRedbStorage::new("master_password").unwrap();
        let test_data = serde_json::json!({
            "user_id": "12345",
            "email": "test@bunkerverse.com",
            "sensitive_data": "secret_info"
        });
        
        let encrypted = storage.encrypt_for_storage(&test_data).unwrap();
        let decrypted: serde_json::Value = storage.decrypt_from_storage(&encrypted).unwrap();
        
        assert_eq!(test_data, decrypted);
    }
}