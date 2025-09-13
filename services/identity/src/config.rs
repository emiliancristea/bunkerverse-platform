use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StubConfiguration {
    pub base: BaseConfig,
    pub dual_mode: DualModeConfig,
    pub latency: LatencyConfig,
    pub errors: ErrorConfig,
    pub data: DataConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BaseConfig {
    pub name: String,
    pub version: String,
    pub port: u16,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DualModeConfig {
    pub enable_crypto: bool,
    pub crypto_response_mode: CryptoResponseMode,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CryptoResponseMode {
    Disabled,
    Error,
    Mock,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LatencyConfig {
    pub min_response_time_ms: u64,
    pub max_response_time_ms: u64,
    pub distribution: LatencyDistribution,
    pub network_condition: NetworkCondition,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LatencyDistribution {
    Uniform,
    Normal,
    Exponential,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NetworkCondition {
    Good,
    Poor,
    Variable,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorConfig {
    pub error_rate: f64,
    pub specific_errors: Vec<SpecificError>,
    pub timeout_rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecificError {
    pub endpoint: String,
    pub status_code: u16,
    pub rate: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataConfig {
    pub dataset: Dataset,
    pub persist_state: bool,
    pub state_reset_interval: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Dataset {
    Minimal,
    Development,
    StressTest,
}

impl Default for StubConfiguration {
    fn default() -> Self {
        Self {
            base: BaseConfig {
                name: "identity-service-stub".to_string(),
                version: "1.0.0".to_string(),
                port: 8083,
                enabled: true,
            },
            dual_mode: DualModeConfig {
                enable_crypto: std::env::var("ENABLE_CRYPTO")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                crypto_response_mode: CryptoResponseMode::Disabled,
            },
            latency: LatencyConfig {
                min_response_time_ms: 10,
                max_response_time_ms: 500,
                distribution: LatencyDistribution::Normal,
                network_condition: NetworkCondition::Good,
            },
            errors: ErrorConfig {
                error_rate: 0.05,
                specific_errors: vec![],
                timeout_rate: 0.01,
            },
            data: DataConfig {
                dataset: Dataset::Development,
                persist_state: false,
                state_reset_interval: "24h".to_string(),
            },
        }
    }
}
