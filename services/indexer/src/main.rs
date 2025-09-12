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
    bunkerverse::services::v1::indexer_service_server::IndexerServiceServer, IndexerGrpcService,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use stub::{IndexerStub, RequestContext, SmartStub};
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
pub struct BlockQuery {
    pub from_block: Option<u64>,
    pub to_block: Option<u64>,
    pub include_transactions: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    pub block_number: u64,
    pub block_hash: String,
    pub parent_hash: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_count: u32,
    pub gas_used: String,
    pub gas_limit: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub tx_hash: String,
    pub block_number: u64,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub gas_price: String,
    pub gas_used: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BlocksResponse {
    pub blocks: Vec<BlockInfo>,
    pub total_count: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize)]
pub struct TransactionsResponse {
    pub transactions: Vec<TransactionInfo>,
    pub total_count: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractState {
    pub contract_address: String,
    pub state_root: String,
    pub code_hash: String,
    pub balance: String,
    pub nonce: u64,
    pub storage: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct IndexingStats {
    pub current_block: u64,
    pub latest_block: u64,
    pub blocks_behind: u64,
    pub indexing_rate: f64,
    pub last_update: DateTime<Utc>,
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
    pub stub: Arc<tokio::sync::Mutex<IndexerStub>>,
}

impl AppState {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(IndexerStub::new(config))),
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

pub async fn get_blocks(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
    Query(block_query): Query<BlockQuery>,
) -> Result<Json<BlocksResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/indexer/blocks", "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if stub.should_inject_error_response() {
        stub.log_response(
            &context,
            "/api/indexer/blocks",
            latency.as_millis() as u64,
            500,
            true,
        );
        let error = ErrorResponse {
            error: "Simulated indexer error".to_string(),
            code: "INTERNAL_ERROR".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
    }

    // Check crypto features
    if let Err(err) = stub.check_crypto_features(&context) {
        stub.log_response(
            &context,
            "/api/indexer/blocks",
            latency.as_millis() as u64,
            403,
            false,
        );
        let error = ErrorResponse {
            error: err,
            code: "FEATURE_NOT_ENABLED".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::FORBIDDEN, Json(error)));
    }

    // Generate mock blocks
    let blocks = vec![
        BlockInfo {
            block_number: 12345,
            block_hash: "0xabc123def456".to_string(),
            parent_hash: "0xdef456abc123".to_string(),
            timestamp: Utc::now(),
            transaction_count: 42,
            gas_used: "8500000".to_string(),
            gas_limit: "12000000".to_string(),
            size: 2048,
        },
        BlockInfo {
            block_number: 12346,
            block_hash: "0xdef456abc123".to_string(),
            parent_hash: "0xabc123def456".to_string(),
            timestamp: Utc::now() - chrono::Duration::seconds(15),
            transaction_count: 38,
            gas_used: "7200000".to_string(),
            gas_limit: "12000000".to_string(),
            size: 1876,
        },
    ];

    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100);

    let response = BlocksResponse {
        blocks,
        total_count: 2,
        page,
        limit,
    };

    stub.log_response(
        &context,
        "/api/indexer/blocks",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn get_block_details(
    State(state): State<AppState>,
    Path(block_number): Path<String>,
) -> Result<Json<BlockInfo>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(
        &context,
        &format!("/api/indexer/blocks/{}", block_number),
        "GET",
    );

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if let Err(err) = stub.check_crypto_features(&context) {
        stub.log_response(
            &context,
            &format!("/api/indexer/blocks/{}", block_number),
            latency.as_millis() as u64,
            403,
            false,
        );
        let error = ErrorResponse {
            error: err,
            code: "FEATURE_NOT_ENABLED".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::FORBIDDEN, Json(error)));
    }

    let block = BlockInfo {
        block_number: block_number.parse().unwrap_or(12345),
        block_hash: "0xabc123def456".to_string(),
        parent_hash: "0xdef456abc123".to_string(),
        timestamp: Utc::now(),
        transaction_count: 42,
        gas_used: "8500000".to_string(),
        gas_limit: "12000000".to_string(),
        size: 2048,
    };

    stub.log_response(
        &context,
        &format!("/api/indexer/blocks/{}", block_number),
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(block))
}

pub async fn get_transactions(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<TransactionsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/indexer/transactions", "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if let Err(err) = stub.check_crypto_features(&context) {
        stub.log_response(
            &context,
            "/api/indexer/transactions",
            latency.as_millis() as u64,
            403,
            false,
        );
        let error = ErrorResponse {
            error: err,
            code: "FEATURE_NOT_ENABLED".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::FORBIDDEN, Json(error)));
    }

    let transactions = vec![TransactionInfo {
        tx_hash: "0x123abc456def".to_string(),
        block_number: 12345,
        from_address: "0xsender123".to_string(),
        to_address: "0xreceiver456".to_string(),
        value: "1000000000000000000".to_string(), // 1 ETH
        gas_price: "20000000000".to_string(),
        gas_used: "21000".to_string(),
        status: "success".to_string(),
        timestamp: Utc::now(),
    }];

    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100);

    let response = TransactionsResponse {
        transactions,
        total_count: 1,
        page,
        limit,
    };

    stub.log_response(
        &context,
        "/api/indexer/transactions",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn get_indexing_status(
    State(state): State<AppState>,
) -> Result<Json<IndexingStats>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/indexer/status", "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if let Err(err) = stub.check_crypto_features(&context) {
        stub.log_response(
            &context,
            "/api/indexer/status",
            latency.as_millis() as u64,
            403,
            false,
        );
        let error = ErrorResponse {
            error: err,
            code: "FEATURE_NOT_ENABLED".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::FORBIDDEN, Json(error)));
    }

    let stats = IndexingStats {
        current_block: 12345,
        latest_block: 12350,
        blocks_behind: 5,
        indexing_rate: 2.5,
        last_update: Utc::now(),
    };

    stub.log_response(
        &context,
        "/api/indexer/status",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(stats))
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
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", config.base.port + 1000).parse()?; // gRPC on port 9082
    let state = AppState::new(config.clone());

    info!(
        service_name = %config.base.name,
        version = %config.base.version,
        http_address = %http_addr,
        grpc_address = %grpc_addr,
        enable_crypto = config.dual_mode.enable_crypto,
        "Starting Indexer Service Smart Stub with HTTP and gRPC servers"
    );

    let app = Router::new()
        // Health and configuration endpoints
        .route("/health", get(health_check))
        .route("/stub/config", get(get_stub_config))
        .route("/stub/reset", post(reset_stub_state))
        // Indexer API endpoints
        .route("/api/indexer/blocks", get(get_blocks))
        .route("/api/indexer/blocks/:block_number", get(get_block_details))
        .route("/api/indexer/transactions", get(get_transactions))
        .route("/api/indexer/status", get(get_indexing_status))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let http_listener = tokio::net::TcpListener::bind(http_addr).await?;

    // gRPC Server
    let grpc_service = IndexerGrpcService::new(config.clone());

    info!("HTTP server ready and listening on {}", http_addr);
    info!("gRPC server ready and listening on {}", grpc_addr);

    // Run both servers concurrently
    let http_server = axum::serve(http_listener, app).with_graceful_shutdown(shutdown_signal());

    let grpc_server = Server::builder()
        .add_service(IndexerServiceServer::new(grpc_service))
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
