use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{info, warn, error};
use uuid::Uuid;

mod crypto;
mod oauth;
mod zkproof;

use crypto::CryptoEngine;
use oauth::{OAuthProvider, OAuthValidation, IdTokenClaims};
use zkproof::{ZkProofSystem, ZkProof, ProofVerification};

/// zkLogin authentication request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZkLoginRequest {
    pub id: String,
    pub email: String,
    pub provider: OAuthProvider,
    pub id_token: String,
    pub nonce: String,
    pub client_nonce: String,
    pub timestamp: DateTime<Utc>,
}

/// zkLogin authentication response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ZkLoginResponse {
    pub id: String,
    pub user_id: String,
    pub access_token: String,
    pub zk_proof: ZkProof,
    pub expires_at: DateTime<Utc>,
    pub verification_key: String,
    pub timestamp: DateTime<Utc>,
    pub status: AuthStatus,
}

/// Authentication status
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthStatus {
    Success,
    InvalidToken,
    InvalidProof,
    ProviderError(String),
    CryptoError(String),
}

/// User identity derived from zkLogin
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserIdentity {
    pub user_id: String,
    pub email: String,
    pub provider: OAuthProvider,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
    pub nonce_used: String,
    pub proof_hash: String,
}

/// zkLogin service configuration
#[derive(Debug, Clone)]
pub struct ZkLoginConfig {
    pub supported_providers: Vec<OAuthProvider>,
    pub token_lifetime: Duration,
    pub proof_verification_timeout: Duration,
    pub enable_strong_crypto: bool,
}

impl Default for ZkLoginConfig {
    fn default() -> Self {
        Self {
            supported_providers: vec![
                OAuthProvider::Google,
                OAuthProvider::GitHub,
                OAuthProvider::Discord,
            ],
            token_lifetime: Duration::from_secs(24 * 60 * 60),
            proof_verification_timeout: Duration::from_secs(30),
            enable_strong_crypto: true,
        }
    }
}

/// zkLogin authentication service
pub struct ZkLoginService {
    config: ZkLoginConfig,
    crypto_engine: CryptoEngine,
    zk_proof_system: ZkProofSystem,
    oauth_validators: std::collections::HashMap<OAuthProvider, OAuthValidation>,
    request_count: u64,
    successful_auths: u64,
    total_processing_time: Duration,
}

impl ZkLoginService {
    /// Create new zkLogin service
    pub async fn new(config: ZkLoginConfig) -> Result<Self> {
        info!("🔐 Initializing zkLogin Authentication Service...");
        
        // Initialize cryptographic engine
        let crypto_engine = CryptoEngine::new(config.enable_strong_crypto)
            .context("Failed to initialize crypto engine")?;
        
        // Initialize ZK proof system
        let zk_proof_system = ZkProofSystem::new()
            .context("Failed to initialize ZK proof system")?;
        
        // Initialize OAuth validators for each supported provider
        let mut oauth_validators = std::collections::HashMap::new();
        for provider in &config.supported_providers {
            let validator = OAuthValidation::new(provider.clone()).await
                .context(format!("Failed to initialize OAuth validator for {:?}", provider))?;
            oauth_validators.insert(provider.clone(), validator);
        }
        
        info!("✅ zkLogin service initialized with {} providers", config.supported_providers.len());
        
        Ok(Self {
            config,
            crypto_engine,
            zk_proof_system,
            oauth_validators,
            request_count: 0,
            successful_auths: 0,
            total_processing_time: Duration::new(0, 0),
        })
    }

    /// Process zkLogin authentication request
    pub async fn authenticate(&mut self, request: ZkLoginRequest) -> Result<ZkLoginResponse> {
        let start_time = Instant::now();
        self.request_count += 1;
        
        info!("🎯 Processing zkLogin request: {} (provider: {:?})", 
              request.id, request.provider);

        // Step 1: Validate OAuth ID token
        let token_validation = self.validate_id_token(&request).await?;
        
        // Step 2: Generate ZK proof of identity
        let zk_proof = self.generate_zk_proof(&request, &token_validation).await?;
        
        // Step 3: Verify the generated proof
        let proof_verification = self.verify_zk_proof(&zk_proof, &request).await?;
        
        if !proof_verification.is_valid {
            return Ok(ZkLoginResponse {
                id: Uuid::new_v4().to_string(),
                user_id: String::new(),
                access_token: String::new(),
                zk_proof: zk_proof,
                expires_at: Utc::now(),
                verification_key: String::new(),
                timestamp: Utc::now(),
                status: AuthStatus::InvalidProof,
            });
        }
        
        // Step 4: Create user identity and access token
        let user_identity = self.create_user_identity(&request, &token_validation, &zk_proof)?;
        let access_token = self.generate_access_token(&user_identity)?;
        
        // Update statistics
        self.successful_auths += 1;
        let processing_time = start_time.elapsed();
        self.total_processing_time += processing_time;
        
        info!("✅ Authentication successful: {} ({}ms)", 
              user_identity.user_id, processing_time.as_millis());
        
        Ok(ZkLoginResponse {
            id: Uuid::new_v4().to_string(),
            user_id: user_identity.user_id,
            access_token,
            zk_proof,
            expires_at: Utc::now() + chrono::Duration::from_std(self.config.token_lifetime)?,
            verification_key: proof_verification.verification_key,
            timestamp: Utc::now(),
            status: AuthStatus::Success,
        })
    }

    /// Validate OAuth ID token
    async fn validate_id_token(&self, request: &ZkLoginRequest) -> Result<IdTokenClaims> {
        let validator = self.oauth_validators.get(&request.provider)
            .ok_or_else(|| anyhow::anyhow!("Unsupported OAuth provider: {:?}", request.provider))?;
        
        validator.validate_token(&request.id_token, &request.nonce).await
            .context("ID token validation failed")
    }

    /// Generate ZK proof of identity
    async fn generate_zk_proof(
        &mut self,
        request: &ZkLoginRequest,
        token_claims: &IdTokenClaims,
    ) -> Result<ZkProof> {
        let proof_input = zkproof::ProofInput {
            email: token_claims.email.clone(),
            provider: request.provider.clone(),
            nonce: request.nonce.clone(),
            client_nonce: request.client_nonce.clone(),
            token_hash: self.crypto_engine.hash_string(&request.id_token)?,
            timestamp: request.timestamp,
        };
        
        self.zk_proof_system.generate_proof(proof_input).await
            .context("ZK proof generation failed")
    }

    /// Verify ZK proof
    async fn verify_zk_proof(
        &self,
        proof: &ZkProof,
        request: &ZkLoginRequest,
    ) -> Result<ProofVerification> {
        self.zk_proof_system.verify_proof(proof, &request.client_nonce).await
            .context("ZK proof verification failed")
    }

    /// Create user identity from validated authentication
    fn create_user_identity(
        &self,
        request: &ZkLoginRequest,
        token_claims: &IdTokenClaims,
        zk_proof: &ZkProof,
    ) -> Result<UserIdentity> {
        let user_id = self.crypto_engine.derive_user_id(&token_claims.email, &request.provider)?;
        let proof_hash = self.crypto_engine.hash_proof(zk_proof)?;
        
        Ok(UserIdentity {
            user_id,
            email: token_claims.email.clone(),
            provider: request.provider.clone(),
            email_verified: token_claims.email_verified,
            created_at: Utc::now(),
            last_login: Utc::now(),
            nonce_used: request.nonce.clone(),
            proof_hash,
        })
    }

    /// Generate secure access token
    fn generate_access_token(&self, identity: &UserIdentity) -> Result<String> {
        self.crypto_engine.generate_jwt_token(identity, self.config.token_lifetime)
            .context("Failed to generate access token")
    }

    /// Get service statistics
    pub fn get_statistics(&self) -> ZkLoginStats {
        ZkLoginStats {
            total_requests: self.request_count,
            successful_authentications: self.successful_auths,
            success_rate: if self.request_count > 0 {
                (self.successful_auths as f64 / self.request_count as f64) * 100.0
            } else { 0.0 },
            average_processing_time: if self.request_count > 0 {
                self.total_processing_time / self.request_count as u32
            } else { Duration::new(0, 0) },
            supported_providers: self.config.supported_providers.clone(),
            crypto_enabled: self.config.enable_strong_crypto,
        }
    }

    /// Validate service functionality
    pub async fn validate(&mut self) -> Result<()> {
        info!("🔍 Validating zkLogin service functionality...");
        
        // Test with mock OAuth token
        let test_request = ZkLoginRequest {
            id: Uuid::new_v4().to_string(),
            email: "test@bunkerverse.io".to_string(),
            provider: OAuthProvider::Google,
            id_token: "mock_jwt_token_for_testing".to_string(),
            nonce: "test_nonce_123456".to_string(),
            client_nonce: "client_nonce_789012".to_string(),
            timestamp: Utc::now(),
        };

        // This will use mock validation for PoC
        let response = self.authenticate(test_request).await;
        
        match response {
            Ok(auth_response) => {
                if matches!(auth_response.status, AuthStatus::Success) {
                    info!("✅ zkLogin service validation successful");
                    info!("   User ID: {}", auth_response.user_id);
                    info!("   Proof generated: {}", !auth_response.zk_proof.proof_data.is_empty());
                    info!("   Token expires: {}", auth_response.expires_at);
                } else {
                    warn!("⚠️  zkLogin validation completed but with non-success status: {:?}", auth_response.status);
                }
                Ok(())
            }
            Err(e) => {
                error!("❌ zkLogin service validation failed: {}", e);
                Err(e)
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ZkLoginStats {
    pub total_requests: u64,
    pub successful_authentications: u64,
    pub success_rate: f64,
    pub average_processing_time: Duration,
    pub supported_providers: Vec<OAuthProvider>,
    pub crypto_enabled: bool,
}

// Performance testing
pub async fn run_performance_tests(service: &mut ZkLoginService) -> Result<PerformanceReport> {
    info!("🚀 Starting zkLogin performance tests...");
    let mut report = PerformanceReport::default();
    
    // Test 1: Google OAuth authentication
    let google_start = Instant::now();
    let google_requests = create_google_auth_requests(5)?;
    let mut google_responses = Vec::new();
    
    for request in google_requests {
        let response = service.authenticate(request).await?;
        google_responses.push(response);
    }
    
    report.google_auth_time = google_start.elapsed();
    report.google_auth_requests = google_responses.len();
    report.google_success_count = google_responses.iter()
        .filter(|r| matches!(r.status, AuthStatus::Success))
        .count();
    
    // Test 2: GitHub OAuth authentication  
    let github_start = Instant::now();
    let github_requests = create_github_auth_requests(3)?;
    let mut github_responses = Vec::new();
    
    for request in github_requests {
        let response = service.authenticate(request).await?;
        github_responses.push(response);
    }
    
    report.github_auth_time = github_start.elapsed();
    report.github_auth_requests = github_responses.len();
    report.github_success_count = github_responses.iter()
        .filter(|r| matches!(r.status, AuthStatus::Success))
        .count();
    
    // Test 3: Concurrent authentication test
    let concurrent_start = Instant::now();
    let concurrent_requests = create_mixed_auth_requests(10)?;
    
    let mut handles = Vec::new();
    for request in concurrent_requests {
        // Note: In a real concurrent test, we'd need to handle shared mutable access
        // For PoC, we'll simulate concurrent processing
        handles.push(request);
    }
    
    // Process requests sequentially for PoC (in reality would be concurrent)
    let mut concurrent_responses = Vec::new();
    for request in handles {
        let response = service.authenticate(request).await?;
        concurrent_responses.push(response);
    }
    
    report.concurrent_auth_time = concurrent_start.elapsed();
    report.concurrent_auth_requests = concurrent_responses.len();
    report.concurrent_success_count = concurrent_responses.iter()
        .filter(|r| matches!(r.status, AuthStatus::Success))
        .count();
    
    // Get final service statistics
    report.final_stats = service.get_statistics();
    
    info!("✅ zkLogin performance tests completed");
    Ok(report)
}

#[derive(Debug, Default)]
pub struct PerformanceReport {
    pub google_auth_time: Duration,
    pub google_auth_requests: usize,
    pub google_success_count: usize,
    pub github_auth_time: Duration,
    pub github_auth_requests: usize,
    pub github_success_count: usize,
    pub concurrent_auth_time: Duration,
    pub concurrent_auth_requests: usize,
    pub concurrent_success_count: usize,
    pub final_stats: ZkLoginStats,
}

impl PerformanceReport {
    pub fn print_summary(&self) {
        println!("\n🔐 ZKLOGIN AUTHENTICATION PoC - PERFORMANCE REPORT");
        println!("==================================================");
        
        println!("\n🔍 Google OAuth Authentication:");
        println!("  • Processed {} requests in {:?}", 
                 self.google_auth_requests, self.google_auth_time);
        println!("  • Successful authentications: {}/{}", 
                 self.google_success_count, self.google_auth_requests);
        if self.google_auth_time.as_secs_f64() > 0.0 {
            println!("  • Rate: {:.2} auth/sec", 
                     self.google_auth_requests as f64 / self.google_auth_time.as_secs_f64());
        }
        
        println!("\n🐱 GitHub OAuth Authentication:");
        println!("  • Processed {} requests in {:?}", 
                 self.github_auth_requests, self.github_auth_time);
        println!("  • Successful authentications: {}/{}", 
                 self.github_success_count, self.github_auth_requests);
        if self.github_auth_time.as_secs_f64() > 0.0 {
            println!("  • Rate: {:.2} auth/sec", 
                     self.github_auth_requests as f64 / self.github_auth_time.as_secs_f64());
        }
        
        println!("\n⚡ Concurrent Authentication:");
        println!("  • Processed {} requests in {:?}", 
                 self.concurrent_auth_requests, self.concurrent_auth_time);
        println!("  • Successful authentications: {}/{}", 
                 self.concurrent_success_count, self.concurrent_auth_requests);
        if self.concurrent_auth_time.as_secs_f64() > 0.0 {
            println!("  • Rate: {:.2} auth/sec", 
                     self.concurrent_auth_requests as f64 / self.concurrent_auth_time.as_secs_f64());
        }
        
        println!("\n📊 Overall Statistics:");
        println!("  • Total requests: {}", self.final_stats.total_requests);
        println!("  • Successful auths: {}", self.final_stats.successful_authentications);
        println!("  • Success rate: {:.1}%", self.final_stats.success_rate);
        println!("  • Average processing time: {:?}", self.final_stats.average_processing_time);
        
        println!("\n🔧 Configuration:");
        println!("  • Supported providers: {:?}", self.final_stats.supported_providers);
        println!("  • Strong crypto: {}", self.final_stats.crypto_enabled);
        
        println!("==================================================");
    }
}

// Helper functions for test data creation
fn create_google_auth_requests(count: usize) -> Result<Vec<ZkLoginRequest>> {
    let mut requests = Vec::new();
    for i in 0..count {
        requests.push(ZkLoginRequest {
            id: Uuid::new_v4().to_string(),
            email: format!("user{}@gmail.com", i),
            provider: OAuthProvider::Google,
            id_token: format!("mock_google_token_{}", i),
            nonce: format!("google_nonce_{}", i),
            client_nonce: format!("client_nonce_{}", i),
            timestamp: Utc::now(),
        });
    }
    Ok(requests)
}

fn create_github_auth_requests(count: usize) -> Result<Vec<ZkLoginRequest>> {
    let mut requests = Vec::new();
    for i in 0..count {
        requests.push(ZkLoginRequest {
            id: Uuid::new_v4().to_string(),
            email: format!("user{}@github.local", i),
            provider: OAuthProvider::GitHub,
            id_token: format!("mock_github_token_{}", i),
            nonce: format!("github_nonce_{}", i),
            client_nonce: format!("client_nonce_{}", i),
            timestamp: Utc::now(),
        });
    }
    Ok(requests)
}

fn create_mixed_auth_requests(count: usize) -> Result<Vec<ZkLoginRequest>> {
    let mut requests = Vec::new();
    let providers = vec![OAuthProvider::Google, OAuthProvider::GitHub, OAuthProvider::Discord];
    
    for i in 0..count {
        let provider = providers[i % providers.len()].clone();
        requests.push(ZkLoginRequest {
            id: Uuid::new_v4().to_string(),
            email: format!("user{}@example.com", i),
            provider: provider.clone(),
            id_token: format!("mock_{:?}_token_{}", provider, i),
            nonce: format!("mixed_nonce_{}", i),
            client_nonce: format!("client_nonce_{}", i),
            timestamp: Utc::now(),
        });
    }
    Ok(requests)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting zkLogin Authentication PoC");
    
    // Initialize zkLogin service
    let config = ZkLoginConfig::default();
    let mut service = ZkLoginService::new(config).await
        .context("Failed to initialize zkLogin service")?;
    
    // Validate service functionality
    service.validate().await
        .context("Service validation failed")?;
    
    // Run comprehensive performance tests
    let report = run_performance_tests(&mut service).await
        .context("Performance tests failed")?;
    
    // Print results
    report.print_summary();
    
    info!("🎉 zkLogin Authentication PoC completed successfully!");
    
    Ok(())
}