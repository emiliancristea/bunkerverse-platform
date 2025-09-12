mod config;
mod grpc_server;
mod stub;

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use config::StubConfiguration;
use grpc_server::{
    bunkerverse::services::v1::ai_data_service_server::AiDataServiceServer, AiDataGrpcService,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use stub::{AiDataStub, RequestContext, SmartStub};
use tokio::signal;
use tonic::transport::Server;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

// API Request/Response Types
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ModelTrainingRequest {
    pub dataset_id: String,
    pub model_type: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct InferenceRequest {
    pub model_id: String,
    pub input_data: serde_json::Value,
    pub options: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
    pub dataset_id: String,
    pub name: String,
    pub description: String,
    pub size: u64,
    pub format: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub model_id: String,
    pub name: String,
    pub model_type: String,
    pub status: String,
    pub accuracy: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub training_completed_at: Option<DateTime<Utc>>,
    pub dataset_id: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct TrainingResponse {
    pub job_id: String,
    pub model_id: String,
    pub status: String,
    pub estimated_completion: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct InferenceResponse {
    pub inference_id: String,
    pub model_id: String,
    pub result: serde_json::Value,
    pub confidence: f64,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct DatasetsResponse {
    pub datasets: Vec<Dataset>,
    pub total_count: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize)]
pub struct ModelsResponse {
    pub models: Vec<Model>,
    pub total_count: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub service_name: String,
    pub version: String,
    pub enable_crypto: bool,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct StubConfigResponse {
    pub config: StubConfiguration,
    pub service_info: stub::ServiceInfo,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub timestamp: DateTime<Utc>,
    pub request_id: String,
}

// Application State
#[derive(Clone)]
pub struct AppState {
    pub stub: Arc<tokio::sync::Mutex<AiDataStub>>,
}

impl AppState {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(AiDataStub::new(config))),
        }
    }

    pub async fn create_context(&self, trace_id: Option<String>) -> RequestContext {
        let stub = self.stub.lock().await;
        RequestContext {
            request_id: Uuid::new_v4().to_string(),
            trace_id,
            timestamp: Utc::now(),
            enable_crypto: stub.get_configuration().dual_mode.enable_crypto,
        }
    }
}

// Handler Functions
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let stub = state.stub.lock().await;
    let service_info = stub.get_service_info();
    let health = stub.health_check();

    let response = HealthResponse {
        status: match health {
            stub::HealthStatus::Healthy => "HEALTHY".to_string(),
            stub::HealthStatus::Degraded => "DEGRADED".to_string(),
            stub::HealthStatus::Unhealthy => "UNHEALTHY".to_string(),
        },
        timestamp: Utc::now(),
        service_name: service_info.name,
        version: service_info.version,
        enable_crypto: stub.get_configuration().dual_mode.enable_crypto,
        details: HashMap::new(),
    };

    Ok(Json(response))
}

pub async fn get_stub_config(
    State(state): State<AppState>,
) -> Result<Json<StubConfigResponse>, StatusCode> {
    let stub = state.stub.lock().await;
    let config = stub.get_configuration().clone();
    let service_info = stub.get_service_info();

    Ok(Json(StubConfigResponse {
        config,
        service_info,
    }))
}

pub async fn reset_stub_state(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut stub = state.stub.lock().await;
    stub.reset_state()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(
        serde_json::json!({"message": "State reset successfully"}),
    ))
}

pub async fn get_datasets(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<DatasetsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/ai-data/datasets", "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if stub.should_inject_error_response() {
        stub.log_response(
            &context,
            "/api/ai-data/datasets",
            latency.as_millis() as u64,
            500,
            true,
        );
        let error = ErrorResponse {
            error: "Simulated AI data error".to_string(),
            code: "INTERNAL_ERROR".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
    }

    let datasets = vec![Dataset {
        dataset_id: "ds_001".to_string(),
        name: "Player Behavior Dataset".to_string(),
        description: "Anonymized player interaction patterns".to_string(),
        size: 1024000,
        format: "parquet".to_string(),
        created_at: Utc::now() - chrono::Duration::days(30),
        updated_at: Utc::now() - chrono::Duration::days(1),
        tags: vec!["behavior".to_string(), "analytics".to_string()],
        metadata: HashMap::new(),
    }];

    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100);

    let response = DatasetsResponse {
        datasets,
        total_count: 1,
        page,
        limit,
    };

    stub.log_response(
        &context,
        "/api/ai-data/datasets",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn get_models(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ModelsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/ai-data/models", "GET");

    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    let models = vec![Model {
        model_id: "model_001".to_string(),
        name: "Player Engagement Predictor".to_string(),
        model_type: "classification".to_string(),
        status: "trained".to_string(),
        accuracy: Some(0.87),
        created_at: Utc::now() - chrono::Duration::days(15),
        training_completed_at: Some(Utc::now() - chrono::Duration::days(10)),
        dataset_id: "ds_001".to_string(),
        version: "1.0.0".to_string(),
    }];

    let response = ModelsResponse {
        models,
        total_count: 1,
        page: pagination.page.unwrap_or(1),
        limit: pagination.limit.unwrap_or(10).min(100),
    };

    stub.log_response(
        &context,
        "/api/ai-data/models",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn train_model(
    State(state): State<AppState>,
    Json(request): Json<ModelTrainingRequest>,
) -> Result<Json<TrainingResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/ai-data/models/train", "POST");

    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    let response = TrainingResponse {
        job_id: Uuid::new_v4().to_string(),
        model_id: Uuid::new_v4().to_string(),
        status: "queued".to_string(),
        estimated_completion: Utc::now() + chrono::Duration::hours(2),
    };

    stub.log_response(
        &context,
        "/api/ai-data/models/train",
        latency.as_millis() as u64,
        202,
        false,
    );
    Ok(Json(response))
}

pub async fn inference(
    State(state): State<AppState>,
    Json(request): Json<InferenceRequest>,
) -> Result<Json<InferenceResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/ai-data/inference", "POST");

    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    let response = InferenceResponse {
        inference_id: Uuid::new_v4().to_string(),
        model_id: request.model_id,
        result: serde_json::json!({"prediction": "high_engagement", "probability": 0.85}),
        confidence: 0.85,
        processing_time_ms: latency.as_millis() as u64,
    };

    stub.log_response(
        &context,
        "/api/ai-data/inference",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing with structured JSON logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = StubConfiguration::default();
    let http_addr: SocketAddr = format!("0.0.0.0:{}", config.base.port).parse()?;
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", config.base.port + 1000).parse()?; // gRPC on port 9084
    let state = AppState::new(config.clone());

    info!(
        service_name = %config.base.name,
        version = %config.base.version,
        http_address = %http_addr,
        grpc_address = %grpc_addr,
        enable_crypto = config.dual_mode.enable_crypto,
        "Starting AI Data Service Smart Stub with HTTP and gRPC servers"
    );

    let app = Router::new()
        // Health and configuration endpoints
        .route("/health", get(health_check))
        .route("/stub/config", get(get_stub_config))
        .route("/stub/reset", post(reset_stub_state))
        // AI Data API endpoints
        .route("/api/ai-data/datasets", get(get_datasets))
        .route("/api/ai-data/models", get(get_models))
        .route("/api/ai-data/models/train", post(train_model))
        .route("/api/ai-data/inference", post(inference))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let http_listener = tokio::net::TcpListener::bind(http_addr).await?;

    // gRPC Server
    let grpc_service = AiDataGrpcService::new(config.clone());

    info!("HTTP server ready and listening on {}", http_addr);
    info!("gRPC server ready and listening on {}", grpc_addr);

    // Run both servers concurrently
    let http_server = axum::serve(http_listener, app).with_graceful_shutdown(shutdown_signal());

    let grpc_server = Server::builder()
        .add_service(AiDataServiceServer::new(grpc_service))
        .serve_with_shutdown(grpc_addr, shutdown_signal());

    // Use tokio::try_join to run both servers concurrently
    tokio::try_join!(http_server, grpc_server)?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down gracefully");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down gracefully");
        },
    }
}
