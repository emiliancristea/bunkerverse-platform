Task 4.5: Rust Payment Service - Secure Stripe Integration & Functional L3 Credit Update
(Principles H, I, J, K, M, R)
Technical Reference
Stripe API Documentation (PaymentIntents, Webhooks, Signature Verification)
stripe-rs, axum, tonic, serde, redis-rs crate documentations
Finalized v0.1 API Contracts (payment_service.proto, OpenAPI spec)
Finalized L3 Smart Contract ABI for the CreditsToken contract

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
To enable the MVE's economy and create a sustainable business model, we need a secure and seamless way for users to purchase the in-verse stable currency, Credits, using real-world money. This task involves fully implementing and hardening a dedicated Rust Payment Service that integrates with Stripe. This service will handle the creation of payment intents for the client and, most critically, securely process incoming webhooks from Stripe to trigger the minting of Credits via a transaction to our L3 smart contracts.
Measurable Objectives
A fully functional and hardened Rust Payment Service is implemented and deployed to the local Docker Compose simulation.
The /credits/purchase/intent endpoint is functional and securely provides a client_secret to the client.
The /stripe/webhook endpoint is implemented with mandatory signature verification and idempotent processing.
A successful payment event from Stripe correctly triggers a CreditsUpdated transaction on the L3, and the user's on-chain balance is updated.
Implementation Guidance
Action: Fully implement and harden the Rust Payment Service (/services/payment/rust/src/) for Credit purchases via Stripe, ensuring it securely updates player balances on the functional L3 Smart Contracts.
1. Implementation Details (Axum-based Service):
a. Dependencies: Add stripe-rs and other necessary crates.
b. Configuration: Securely load the Stripe API secret key and webhook signing secret from the .env file for the local Docker Compose simulation.
c. API Endpoints (OpenAPI defined):
POST /credits/purchase/intent:
Validates the player's JWT and the requested credit_package_id.
Creates a Stripe PaymentIntent with the correct EUR amount, currency, and metadata (e.g., player_on_chain_address).
Returns the client_secret from the PaymentIntent to the client.
POST /stripe/webhook:
Signature Verification (Critical Security): First and most critical step: verify the Stripe-Signature header to ensure the webhook is genuinely from Stripe. Reject any request that fails verification.
Idempotency: Check if the Stripe event.id has been processed before (e.g., by checking a Redis cache or a DB table). If so, return 200 OK immediately to acknowledge receipt but prevent duplicate processing.
Process payment_intent.succeeded event: If the event is a successful payment, extract the player_on_chain_address from the metadata. Construct a CreditsUpdated transaction intent.
Submit to L3: Submit the CreditsUpdated intent to the L3 sequencer (e.g., via the Transaction Submission Service). The payload should include a unique reference ID (like the Stripe payment_intent ID) so the L3 smart contract can also perform idempotency checks if needed.
Record the processed event.id in Redis/DB to prevent duplicates.

## Complete Stripe Webhook Verification Implementation

### Webhook Signature Verification
```rust
// webhook_handler.rs
use stripe::{Webhook, WebhookError};
use axum::{
    extract::{Query, State},
    headers::{HeaderMap, HeaderValue},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{error, info, warn};

pub struct WebhookHandler {
    webhook_secret: String,
    idempotency_store: Arc<dyn IdempotencyStore>,
    payment_processor: Arc<dyn PaymentProcessor>,
}

impl WebhookHandler {
    pub fn new(
        webhook_secret: String,
        idempotency_store: Arc<dyn IdempotencyStore>,
        payment_processor: Arc<dyn PaymentProcessor>,
    ) -> Self {
        Self {
            webhook_secret,
            idempotency_store,
            payment_processor,
        }
    }
    
    pub async fn handle_webhook(
        &self,
        headers: HeaderMap,
        body: Vec<u8>,
    ) -> Result<Response, WebhookError> {
        // Step 1: Extract Stripe signature header
        let signature = headers
            .get("stripe-signature")
            .and_then(|v| v.to_str().ok())
            .ok_or(WebhookError::MissingSignature)?;
            
        // Step 2: Verify webhook signature
        let payload_str = std::str::from_utf8(&body)
            .map_err(|_| WebhookError::InvalidPayload)?;
            
        let event = Webhook::construct_event(
            payload_str,
            signature,
            &self.webhook_secret,
        ).map_err(|e| {
            error!("Webhook signature verification failed: {:?}", e);
            WebhookError::InvalidSignature
        })?;
        
        info!("Webhook received: {} ({})", event.type_, event.id);
        
        // Step 3: Check idempotency
        if self.idempotency_store.has_processed(&event.id).await? {
            info!("Event {} already processed, returning OK", event.id);
            return Ok(StatusCode::OK.into_response());
        }
        
        // Step 4: Process event based on type
        let result = match event.type_.as_str() {
            "payment_intent.succeeded" => {
                self.handle_payment_succeeded(event).await
            },
            "payment_intent.payment_failed" => {
                self.handle_payment_failed(event).await
            },
            "payment_intent.canceled" => {
                self.handle_payment_canceled(event).await
            },
            _ => {
                info!("Unhandled webhook event type: {}", event.type_);
                Ok(()) // Return OK for unhandled events
            }
        };
        
        match result {
            Ok(_) => {
                // Mark as processed only on success
                self.idempotency_store.mark_processed(&event.id).await?;
                info!("Successfully processed webhook event: {}", event.id);
                Ok(StatusCode::OK.into_response())
            },
            Err(e) => {
                error!("Failed to process webhook event {}: {:?}", event.id, e);
                // Don't mark as processed on failure - allow retry
                Err(WebhookError::ProcessingFailed(e.to_string()))
            }
        }
    }
    
    async fn handle_payment_succeeded(&self, event: stripe::Event) -> Result<(), ProcessingError> {
        let payment_intent: stripe::PaymentIntent = event.data.object
            .deserialize()
            .map_err(|e| ProcessingError::InvalidEventData(e.to_string()))?;
            
        info!("Processing successful payment: {}", payment_intent.id);
        
        // Extract metadata
        let metadata = payment_intent.metadata.unwrap_or_default();
        let player_address = metadata.get("player_on_chain_address")
            .ok_or(ProcessingError::MissingMetadata("player_on_chain_address"))?;
            
        let credit_package_id = metadata.get("credit_package_id")
            .ok_or(ProcessingError::MissingMetadata("credit_package_id"))?;
            
        // Calculate credits amount based on package
        let credits_amount = self.calculate_credits_from_package(credit_package_id)?;
        
        // Submit to L3
        self.payment_processor.process_credit_purchase(
            payment_intent.id.as_str(),
            player_address,
            credits_amount,
            payment_intent.amount,
        ).await?;
        
        Ok(())
    }
    
    async fn handle_payment_failed(&self, event: stripe::Event) -> Result<(), ProcessingError> {
        let payment_intent: stripe::PaymentIntent = event.data.object
            .deserialize()
            .map_err(|e| ProcessingError::InvalidEventData(e.to_string()))?;
            
        warn!("Payment failed: {} - {}", 
              payment_intent.id, 
              payment_intent.last_payment_error
                  .map(|e| e.message.unwrap_or_default())
                  .unwrap_or_default());
                  
        // Log for analytics/alerting
        self.payment_processor.record_payment_failure(
            payment_intent.id.as_str(),
            payment_intent.last_payment_error,
        ).await?;
        
        Ok(())
    }
    
    async fn handle_payment_canceled(&self, event: stripe::Event) -> Result<(), ProcessingError> {
        let payment_intent: stripe::PaymentIntent = event.data.object
            .deserialize()
            .map_err(|e| ProcessingError::InvalidEventData(e.to_string()))?;
            
        info!("Payment canceled: {}", payment_intent.id);
        
        // Clean up any pending transactions
        self.payment_processor.cancel_pending_transaction(
            payment_intent.id.as_str()
        ).await?;
        
        Ok(())
    }
    
    fn calculate_credits_from_package(&self, package_id: &str) -> Result<u64, ProcessingError> {
        // In production, load from database
        let packages = HashMap::from([
            ("small", 1000u64),
            ("medium", 5000u64),
            ("large", 15000u64),
            ("premium", 50000u64),
        ]);
        
        packages.get(package_id)
            .copied()
            .ok_or_else(|| ProcessingError::InvalidPackage(package_id.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WebhookError {
    #[error("Missing Stripe signature header")]
    MissingSignature,
    #[error("Invalid webhook signature")]
    InvalidSignature,
    #[error("Invalid webhook payload")]
    InvalidPayload,
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Idempotency store error: {0}")]
    IdempotencyError(#[from] IdempotencyError),
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Invalid event data: {0}")]
    InvalidEventData(String),
    #[error("Missing required metadata: {0}")]
    MissingMetadata(&'static str),
    #[error("Invalid credit package: {0}")]
    InvalidPackage(String),
    #[error("L3 transaction failed: {0}")]
    L3TransactionFailed(String),
}
```

## Idempotency Key Patterns with Database Schema

### Redis-based Idempotency Store
```rust
// idempotency.rs
use async_trait::async_trait;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait IdempotencyStore: Send + Sync {
    async fn has_processed(&self, event_id: &str) -> Result<bool, IdempotencyError>;
    async fn mark_processed(&self, event_id: &str) -> Result<(), IdempotencyError>;
    async fn get_processing_result(&self, event_id: &str) -> Result<Option<ProcessingResult>, IdempotencyError>;
    async fn store_processing_result(&self, event_id: &str, result: ProcessingResult) -> Result<(), IdempotencyError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub transaction_id: Option<String>,
    pub processed_at: DateTime<Utc>,
}

pub struct RedisIdempotencyStore {
    client: redis::Client,
    ttl: Duration, // How long to keep processed events
}

impl RedisIdempotencyStore {
    pub fn new(redis_url: &str, ttl: Duration) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self { client, ttl })
    }
    
    fn processed_key(&self, event_id: &str) -> String {
        format!("webhook:processed:{}", event_id)
    }
    
    fn result_key(&self, event_id: &str) -> String {
        format!("webhook:result:{}", event_id)
    }
}

#[async_trait]
impl IdempotencyStore for RedisIdempotencyStore {
    async fn has_processed(&self, event_id: &str) -> Result<bool, IdempotencyError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = self.processed_key(event_id);
        
        let exists: bool = conn.exists(&key).await?;
        Ok(exists)
    }
    
    async fn mark_processed(&self, event_id: &str) -> Result<(), IdempotencyError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = self.processed_key(event_id);
        let ttl_seconds = self.ttl.as_secs() as usize;
        
        let _: () = conn.setex(&key, ttl_seconds, "processed").await?;
        Ok(())
    }
    
    async fn get_processing_result(&self, event_id: &str) -> Result<Option<ProcessingResult>, IdempotencyError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = self.result_key(event_id);
        
        let result: Option<String> = conn.get(&key).await?;
        
        match result {
            Some(json_str) => {
                let result: ProcessingResult = serde_json::from_str(&json_str)
                    .map_err(|e| IdempotencyError::SerializationError(e.to_string()))?;
                Ok(Some(result))
            },
            None => Ok(None)
        }
    }
    
    async fn store_processing_result(&self, event_id: &str, result: ProcessingResult) -> Result<(), IdempotencyError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = self.result_key(event_id);
        let ttl_seconds = self.ttl.as_secs() as usize;
        
        let json_str = serde_json::to_string(&result)
            .map_err(|e| IdempotencyError::SerializationError(e.to_string()))?;
            
        let _: () = conn.setex(&key, ttl_seconds, json_str).await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IdempotencyError {
    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}
```

### Database Schema for Persistent Idempotency
```sql
-- webhook_events.sql
CREATE TABLE webhook_events (
    id BIGSERIAL PRIMARY KEY,
    event_id VARCHAR(255) UNIQUE NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    processing_status ENUM('pending', 'processing', 'completed', 'failed') NOT NULL DEFAULT 'pending',
    payload JSONB NOT NULL,
    processing_result JSONB NULL,
    transaction_id VARCHAR(255) NULL,
    error_message TEXT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMP WITH TIME ZONE NULL,
    last_retry_at TIMESTAMP WITH TIME ZONE NULL
);

-- Indexes for efficient querying
CREATE INDEX idx_webhook_events_event_id ON webhook_events(event_id);
CREATE INDEX idx_webhook_events_status ON webhook_events(processing_status);
CREATE INDEX idx_webhook_events_type ON webhook_events(event_type);
CREATE INDEX idx_webhook_events_created_at ON webhook_events(created_at);

-- For cleanup of old processed events
CREATE INDEX idx_webhook_events_cleanup ON webhook_events(processing_status, processed_at) 
    WHERE processing_status IN ('completed', 'failed');
```
2. Security Hardening:
o Strict input validation on all API endpoints.
o Idempotency is crucial for both the webhook handler and potentially the smart contract logic to prevent double-spending or double-crediting.
o Protect Stripe keys rigorously; they should never be exposed or hardcoded.

## Secret Management Procedures

### Local Development Environment
```bash
# .env.example
STRIPE_SECRET_KEY=sk_test_...
STRIPE_PUBLISHABLE_KEY=pk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...
DATABASE_URL=postgresql://user:pass@localhost/bunkerverse_payment
REDIS_URL=redis://localhost:6379
L3_RPC_ENDPOINT=http://localhost:8545
L3_PRIVATE_KEY=0x...
JWT_SECRET=your-jwt-secret-key
```

### Production Secret Management (AWS)
```rust
// secrets_manager.rs
use aws_sdk_secretsmanager::{Client as SecretsClient, Error as SecretsError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentServiceSecrets {
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub database_url: String,
    pub redis_url: String,
    pub l3_rpc_endpoint: String,
    pub l3_private_key: String,
    pub jwt_secret: String,
}

pub struct SecretsManager {
    client: SecretsClient,
    cache: RwLock<HashMap<String, (PaymentServiceSecrets, Instant)>>,
    cache_ttl: Duration,
}

impl SecretsManager {
    pub async fn new() -> Result<Self, SecretsError> {
        let config = aws_config::load_from_env().await;
        let client = SecretsClient::new(&config);
        
        Ok(Self {
            client,
            cache: RwLock::new(HashMap::new()),
            cache_ttl: Duration::from_secs(300), // 5 minutes
        })
    }
    
    pub async fn get_secrets(&self, secret_name: &str) -> Result<PaymentServiceSecrets, SecretsError> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some((secrets, timestamp)) = cache.get(secret_name) {
                if timestamp.elapsed() < self.cache_ttl {
                    return Ok(secrets.clone());
                }
            }
        }
        
        // Fetch from AWS Secrets Manager
        let response = self.client
            .get_secret_value()
            .secret_id(secret_name)
            .send()
            .await?;
            
        let secret_string = response.secret_string()
            .ok_or_else(|| SecretsError::Unhandled("No secret string found".into()))?;
            
        let secrets: PaymentServiceSecrets = serde_json::from_str(secret_string)
            .map_err(|e| SecretsError::Unhandled(format!("JSON parse error: {}", e).into()))?;
            
        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(secret_name.to_string(), (secrets.clone(), Instant::now()));
        }
        
        Ok(secrets)
    }
    
    pub async fn refresh_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

// Environment-specific configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub environment: Environment,
    pub secrets_name: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Config {
    pub fn from_env() -> Self {
        let environment = match std::env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str() {
            "production" => Environment::Production,
            "staging" => Environment::Staging,
            _ => Environment::Development,
        };
        
        let secrets_name = match environment {
            Environment::Production => "bunkerverse/payment-service/prod",
            Environment::Staging => "bunkerverse/payment-service/staging",
            Environment::Development => "bunkerverse/payment-service/dev",
        }.to_string();
        
        Self {
            environment,
            secrets_name,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            log_level: std::env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        }
    }
}
```

## Payment Retry Logic and Error Handling

### Robust Payment Processing with Retries
```rust
// payment_processor.rs
use async_trait::async_trait;
use tokio::time::{sleep, Duration, timeout};
use tracing::{error, info, warn};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait PaymentProcessor: Send + Sync {
    async fn process_credit_purchase(
        &self,
        payment_intent_id: &str,
        player_address: &str,
        credits_amount: u64,
        payment_amount: i64,
    ) -> Result<String, PaymentError>;
    
    async fn record_payment_failure(
        &self,
        payment_intent_id: &str,
        error: Option<stripe::PaymentIntentLastPaymentError>,
    ) -> Result<(), PaymentError>;
    
    async fn cancel_pending_transaction(
        &self,
        payment_intent_id: &str,
    ) -> Result<(), PaymentError>;
}

pub struct RobustPaymentProcessor {
    l3_client: Arc<dyn L3Client>,
    database: Arc<dyn PaymentDatabase>,
    retry_config: RetryConfig,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub timeout: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            timeout: Duration::from_secs(60),
        }
    }
}

#[async_trait]
impl PaymentProcessor for RobustPaymentProcessor {
    async fn process_credit_purchase(
        &self,
        payment_intent_id: &str,
        player_address: &str,
        credits_amount: u64,
        payment_amount: i64,
    ) -> Result<String, PaymentError> {
        info!("Processing credit purchase: {} for {} credits", 
              payment_intent_id, credits_amount);
              
        // Record payment attempt
        self.database.record_payment_attempt(
            payment_intent_id,
            player_address,
            credits_amount,
            payment_amount,
        ).await?;
        
        // Retry logic for L3 transaction
        let transaction_result = self.retry_with_backoff(
            || self.submit_l3_transaction(payment_intent_id, player_address, credits_amount),
            &self.retry_config,
        ).await;
        
        match transaction_result {
            Ok(tx_hash) => {
                info!("Credit purchase successful: {} -> {}", payment_intent_id, tx_hash);
                
                // Update database with success
                self.database.mark_payment_successful(
                    payment_intent_id,
                    &tx_hash,
                ).await?;
                
                Ok(tx_hash)
            },
            Err(e) => {
                error!("Credit purchase failed after retries: {} - {:?}", 
                       payment_intent_id, e);
                       
                // Update database with failure
                self.database.mark_payment_failed(
                    payment_intent_id,
                    &e.to_string(),
                ).await?;
                
                Err(e)
            }
        }
    }
    
    async fn record_payment_failure(
        &self,
        payment_intent_id: &str,
        error: Option<stripe::PaymentIntentLastPaymentError>,
    ) -> Result<(), PaymentError> {
        let error_message = error
            .and_then(|e| e.message)
            .unwrap_or_else(|| "Unknown payment error".to_string());
            
        self.database.record_stripe_failure(
            payment_intent_id,
            &error_message,
        ).await?;
        
        // Could trigger notification/alerting here
        warn!("Stripe payment failure recorded: {} - {}", 
              payment_intent_id, error_message);
        
        Ok(())
    }
    
    async fn cancel_pending_transaction(
        &self,
        payment_intent_id: &str,
    ) -> Result<(), PaymentError> {
        // Cancel any pending L3 transactions
        if let Ok(pending_tx) = self.database.get_pending_transaction(payment_intent_id).await {
            if let Some(tx_hash) = pending_tx.transaction_hash {
                // In a real implementation, you might try to cancel the transaction
                // For now, just mark as cancelled
                info!("Marking transaction as cancelled: {}", tx_hash);
            }
        }
        
        self.database.cancel_payment(payment_intent_id).await?;
        Ok(())
    }
}

impl RobustPaymentProcessor {
    async fn submit_l3_transaction(
        &self,
        payment_intent_id: &str,
        player_address: &str,
        credits_amount: u64,
    ) -> Result<String, PaymentError> {
        // Create L3 transaction with timeout
        let tx_future = self.l3_client.mint_credits(
            player_address,
            credits_amount,
            payment_intent_id, // Use as reference ID
        );
        
        timeout(self.retry_config.timeout, tx_future)
            .await
            .map_err(|_| PaymentError::TransactionTimeout)?
    }
    
    async fn retry_with_backoff<T, F, Fut>(
        &self,
        mut operation: F,
        config: &RetryConfig,
    ) -> Result<T, PaymentError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, PaymentError>>,
    {
        let mut attempt = 0;
        let mut delay = config.base_delay;
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= config.max_attempts - 1 => return Err(e),
                Err(e) => {
                    attempt += 1;
                    warn!("Operation failed (attempt {}/{}): {:?}", 
                          attempt, config.max_attempts, e);
                    
                    // Don't retry certain types of errors
                    if matches!(e, PaymentError::InvalidAddress | PaymentError::InvalidAmount) {
                        return Err(e);
                    }
                    
                    sleep(delay).await;
                    
                    // Exponential backoff with jitter
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * config.backoff_multiplier) as u64
                        ),
                        config.max_delay
                    );
                    
                    // Add jitter (±25%)
                    let jitter = fastrand::f64() * 0.5 - 0.25; // -25% to +25%
                    let jitter_millis = (delay.as_millis() as f64 * jitter) as u64;
                    delay = Duration::from_millis(
                        delay.as_millis().saturating_add(jitter_millis as u128) as u64
                    );
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Invalid player address format")]
    InvalidAddress,
    #[error("Invalid credit amount")]
    InvalidAmount,
    #[error("L3 transaction failed: {0}")]
    L3TransactionFailed(String),
    #[error("Transaction timeout")]
    TransactionTimeout,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
}
```
3. Update docs/progress_logs/progress_phase_4.md:
o Log the Rust Payment Service implementation.
o Detail the Stripe API/webhook security measures (signature verification, idempotency) and the on-chain CreditsUpdated transaction submission flow.
o Document the comprehensive error handling and retry mechanisms.
o Record the secret management procedures for different environments.
o Log the database schema for webhook idempotency and audit trails.
Design Rationale
A dedicated microservice for payments isolates the highly sensitive logic and credentials associated with a third-party payment provider (Stripe). The webhook pattern is the standard, asynchronous way to handle payment confirmations. Mandatory signature verification and idempotent processing are non-negotiable security best practices for any webhook handler that triggers a financial transaction, preventing fraudulent requests and duplicate processing.
Operational Considerations
Local-First: The service will run in the local Docker Compose simulation. Stripe's CLI can be used to forward test webhooks from Stripe's servers to the service running on localhost. Secrets will be managed in the .env file.
Cloud-Ready: In production, this service will be a critical, hardened component. Its endpoint will be exposed to the internet to receive webhooks from Stripe. Its Stripe secrets will be loaded from AWS Secrets Manager. Monitoring and alerting for webhook failures will be essential.
Verification & Validation Criteria
Integration tests with a mock Stripe service (simulating success, failure, and duplicate webhooks) and a mock L3 sequencer demonstrate secure and idempotent processing.
An end-to-end test in the local simulation using Stripe's test mode and CLI forwarding successfully results in a CreditsUpdated transaction being processed on the local L3.
Testing Methodologies
Unit Tests: For the logic within the webhook handler (e.g., event parsing, idempotency check logic).
Integration Tests: Use a tool like wiremock-rs to create a mock Stripe service. Write tests that: 1) call the /purchase/intent endpoint, 2) send mock success, failure, and duplicate webhook events to the /stripe/webhook endpoint, and 3) verify that only the single, successful event results in a transaction being submitted to a mocked L3 client.
Version Control Strategy
Branching: The Payment Service will be developed on a feature/payment-service-stripe branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
A mandatory, in-depth security review of the /stripe/webhook endpoint is required before this phase can be completed.
The review must focus on the correctness of the signature verification and the robustness of the idempotency logic.
The secure handling of Stripe API and webhook secrets is a critical checkpoint.
ReviewedBy: Backend Lead, Security Lead, Finance/Ops stakeholders.
ReviewOutcome: Approved.
ValidationMethod: Integration tests with a mock Stripe service and a mock L3 sequencer demonstrate secure and idempotent processing.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.5: Implemented Secure Rust Payment Service (Stripe Integration, Functional L3 Credit Updates, Idempotency)." @Phase4/