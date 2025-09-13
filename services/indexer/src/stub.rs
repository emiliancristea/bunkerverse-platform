use crate::config::{CryptoResponseMode, LatencyDistribution, NetworkCondition, StubConfiguration};
use anyhow::Result;
use chrono::{DateTime, Utc};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub status: String,
    pub uptime: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    pub request_id: String,
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub enable_crypto: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub latency_ms: u64,
    pub injected_error: bool,
}

pub trait SmartStub {
    fn get_service_info(&self) -> ServiceInfo;
    fn health_check(&self) -> HealthStatus;
    fn reset_state(&mut self) -> Result<()>;
    fn get_configuration(&self) -> &StubConfiguration;
    fn set_configuration(&mut self, config: StubConfiguration) -> Result<()>;
}

pub trait ResponseGenerator<TRequest, TResponse> {
    fn generate_response(
        &self,
        request: TRequest,
        context: RequestContext,
    ) -> StubResponse<TResponse>;
    fn should_inject_error(&self, request: &TRequest, context: &RequestContext) -> bool;
    fn calculate_latency(&self, request: &TRequest, context: &RequestContext) -> Duration;
}

pub trait StateManager {
    fn get_state<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T>;
    fn set_state<T: Serialize>(&mut self, key: &str, value: &T) -> Result<()>;
    fn clear_state(&mut self, key: &str) -> Result<()>;
    fn reset_all_state(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct IndexerStub {
    config: StubConfiguration,
    state: Arc<Mutex<HashMap<String, String>>>,
    start_time: DateTime<Utc>,
}

impl IndexerStub {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(HashMap::new())),
            start_time: Utc::now(),
        }
    }

    pub fn calculate_response_latency(&self) -> Duration {
        let mut rng = thread_rng();
        let base_latency = match self.config.latency.distribution {
            LatencyDistribution::Uniform => rng.gen_range(
                self.config.latency.min_response_time_ms..=self.config.latency.max_response_time_ms,
            ),
            LatencyDistribution::Normal => {
                let mean = (self.config.latency.min_response_time_ms
                    + self.config.latency.max_response_time_ms)
                    / 2;
                let std_dev = (self.config.latency.max_response_time_ms
                    - self.config.latency.min_response_time_ms)
                    / 4;
                let normal = rand_distr::Normal::new(mean as f64, std_dev as f64).unwrap();
                rng.sample(normal)
                    .max(self.config.latency.min_response_time_ms as f64) as u64
            }
            LatencyDistribution::Exponential => {
                let lambda = 1.0 / (self.config.latency.max_response_time_ms as f64);
                let exp = rand_distr::Exp::new(lambda).unwrap();
                (rng.sample(exp) + self.config.latency.min_response_time_ms as f64) as u64
            }
        };

        let network_multiplier = match self.config.latency.network_condition {
            NetworkCondition::Good => 1.0,
            NetworkCondition::Poor => 2.5,
            NetworkCondition::Variable => rng.gen_range(0.5..3.0),
        };

        Duration::from_millis((base_latency as f64 * network_multiplier) as u64)
    }

    pub fn should_inject_error_response(&self) -> bool {
        let mut rng = thread_rng();
        rng.gen::<f64>() < self.config.errors.error_rate
    }

    pub fn log_request(&self, context: &RequestContext, endpoint: &str, method: &str) {
        info!(
            timestamp = %context.timestamp,
            stub_name = %self.config.base.name,
            stub_version = %self.config.base.version,
            event_type = "request_received",
            endpoint = endpoint,
            method = method,
            request_id = %context.request_id,
            enable_crypto = context.enable_crypto,
            "Request received"
        );
    }

    pub fn log_response(
        &self,
        context: &RequestContext,
        endpoint: &str,
        latency_ms: u64,
        status: u16,
        error_injected: bool,
    ) {
        info!(
            timestamp = %Utc::now(),
            stub_name = %self.config.base.name,
            stub_version = %self.config.base.version,
            event_type = "response_sent",
            endpoint = endpoint,
            request_id = %context.request_id,
            enable_crypto = context.enable_crypto,
            simulated_latency_ms = latency_ms,
            response_status = status,
            error_injected = error_injected,
            "Response sent"
        );
    }

    pub fn check_crypto_features(&self, context: &RequestContext) -> Result<(), String> {
        if !context.enable_crypto {
            match self.config.dual_mode.crypto_response_mode {
                CryptoResponseMode::Disabled => Err("FEATURE_NOT_ENABLED".to_string()),
                CryptoResponseMode::Error => {
                    Err("Blockchain features are not available".to_string())
                }
                CryptoResponseMode::Mock => Ok(()),
            }
        } else {
            Ok(())
        }
    }
}

impl SmartStub for IndexerStub {
    fn get_service_info(&self) -> ServiceInfo {
        ServiceInfo {
            name: self.config.base.name.clone(),
            version: self.config.base.version.clone(),
            status: if self.config.base.enabled {
                "active"
            } else {
                "inactive"
            }
            .to_string(),
            uptime: Utc::now()
                .signed_duration_since(self.start_time)
                .to_std()
                .unwrap_or_default(),
        }
    }

    fn health_check(&self) -> HealthStatus {
        if self.config.base.enabled {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }

    fn reset_state(&mut self) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.clear();
        info!(
            stub_name = %self.config.base.name,
            event_type = "state_reset",
            "State reset completed"
        );
        Ok(())
    }

    fn get_configuration(&self) -> &StubConfiguration {
        &self.config
    }

    fn set_configuration(&mut self, config: StubConfiguration) -> Result<()> {
        let old_crypto = self.config.dual_mode.enable_crypto;
        let new_crypto = config.dual_mode.enable_crypto;

        self.config = config;

        if old_crypto != new_crypto {
            info!(
                stub_name = %self.config.base.name,
                event_type = "mode_switched",
                enable_crypto = new_crypto,
                "Dual-mode configuration changed"
            );
        }

        info!(
            stub_name = %self.config.base.name,
            event_type = "config_updated",
            "Configuration updated"
        );

        Ok(())
    }
}

impl StateManager for IndexerStub {
    fn get_state<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        let state = self.state.lock().unwrap();
        state.get(key).and_then(|v| serde_json::from_str(v).ok())
    }

    fn set_state<T: Serialize>(&mut self, key: &str, value: &T) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let json = serde_json::to_string(value)?;
        state.insert(key.to_string(), json);

        info!(
            stub_name = %self.config.base.name,
            event_type = "state_updated",
            key = key,
            "State updated"
        );

        Ok(())
    }

    fn clear_state(&mut self, key: &str) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.remove(key);
        Ok(())
    }

    fn reset_all_state(&mut self) -> Result<()> {
        self.reset_state()
    }
}
