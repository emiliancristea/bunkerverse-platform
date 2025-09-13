use anyhow::Result;
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{error, info};
use uuid::Uuid;

/// Configuration for smart stub behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubConfiguration {
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub error_probability: f64,
    pub dual_mode: DualModeConfig,
}

/// Configuration for dual-mode operation (crypto vs non-crypto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DualModeConfig {
    pub enable_crypto: bool,
    pub show_crypto: bool,
}

/// Context for each request processed by a smart stub
#[derive(Debug)]
pub struct RequestContext {
    pub request_id: String,
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub enable_crypto: bool,
}

/// Core trait that all smart stubs must implement
pub trait SmartStub {
    /// Get the current configuration for this stub
    fn get_configuration(&self) -> &StubConfiguration;
    
    /// Determine if an error should be injected based on error probability
    fn should_inject_error_response(&self) -> bool {
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < self.get_configuration().error_probability
    }

    /// Calculate simulated response latency
    fn calculate_response_latency(&self) -> Duration {
        let config = self.get_configuration();
        let mut rng = rand::thread_rng();
        let latency_ms = rng.gen_range(config.min_latency_ms..=config.max_latency_ms);
        Duration::from_millis(latency_ms)
    }

    /// Log response details in structured JSON format
    fn log_response(
        &self,
        context: &RequestContext,
        method: &str,
        latency_ms: u64,
        status_code: u32,
        is_error: bool,
    ) {
        let log = serde_json::json!({
            "request_id": context.request_id,
            "trace_id": context.trace_id,
            "method": method,
            "timestamp": context.timestamp.to_rfc3339(),
            "latency_ms": latency_ms,
            "status_code": status_code,
            "is_error": is_error,
            "enable_crypto": context.enable_crypto
        });

        if is_error {
            error!(response = %log);
        } else {
            info!(response = %log);
        }
    }

    /// Generate mock transaction hash for crypto operations
    fn generate_mock_transaction_hash(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        format!("0x{}", hex::encode(bytes))
    }

    /// Generate mock Ethereum address
    fn generate_mock_eth_address(&self) -> String {
        let mut rng = rand::thread_rng();
        let mut bytes = [0u8; 20];
        rng.fill(&mut bytes);
        format!("0x{}", hex::encode(bytes))
    }
}

impl Default for StubConfiguration {
    fn default() -> Self {
        Self {
            min_latency_ms: 10,
            max_latency_ms: 100,
            error_probability: 0.01,
            dual_mode: DualModeConfig {
                enable_crypto: false,
                show_crypto: false,
            },
        }
    }
}