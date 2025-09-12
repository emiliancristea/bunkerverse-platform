mod config;
mod grpc_server;
mod stub;

use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use config::StubConfiguration;
use grpc_server::{
    bunkerverse::services::v1::identity_service_server::IdentityServiceServer, IdentityGrpcService,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use stub::{IdentityStub, RequestContext, SmartStub};
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
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TokenValidationRequest {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub wallet_address: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub message: String,
    pub verification_required: bool,
}

#[derive(Debug, Serialize)]
pub struct TokenValidationResponse {
    pub valid: bool,
    pub user_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

#[derive(Debug, Serialize)]
pub struct UsersResponse {
    pub users: Vec<User>,
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
    pub stub: Arc<tokio::sync::Mutex<IdentityStub>>,
}

impl AppState {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(IdentityStub::new(config))),
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

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/identity/auth/login", "POST");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if stub.should_inject_error_response() {
        stub.log_response(
            &context,
            "/api/identity/auth/login",
            latency.as_millis() as u64,
            401,
            true,
        );
        let error = ErrorResponse {
            error: "Invalid credentials".to_string(),
            code: "UNAUTHORIZED".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::UNAUTHORIZED, Json(error)));
    }

    // Generate mock user based on dual-mode configuration
    let user = User {
        user_id: Uuid::new_v4().to_string(),
        username: request.username.clone(),
        email: format!("{}@example.com", request.username),
        wallet_address: if context.enable_crypto {
            request
                .wallet_address
                .or_else(|| Some("0x1234567890abcdef".to_string()))
        } else {
            None
        },
        roles: vec!["user".to_string()],
        permissions: vec!["read:profile".to_string(), "update:profile".to_string()],
        is_active: true,
        created_at: Utc::now() - chrono::Duration::days(30),
        last_login: Some(Utc::now()),
    };

    let response = LoginResponse {
        access_token: "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.mock_access_token".to_string(),
        refresh_token: "mock_refresh_token_1234567890".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        user,
    };

    stub.log_response(
        &context,
        "/api/identity/auth/login",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/identity/auth/register", "POST");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if stub.should_inject_error_response() {
        stub.log_response(
            &context,
            "/api/identity/auth/register",
            latency.as_millis() as u64,
            409,
            true,
        );
        let error = ErrorResponse {
            error: "Username already exists".to_string(),
            code: "CONFLICT".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::CONFLICT, Json(error)));
    }

    let response = RegisterResponse {
        user_id: Uuid::new_v4().to_string(),
        message: "User registered successfully".to_string(),
        verification_required: !context.enable_crypto, // Email verification required in MVE mode
    };

    stub.log_response(
        &context,
        "/api/identity/auth/register",
        latency.as_millis() as u64,
        201,
        false,
    );
    Ok(Json(response))
}

pub async fn validate_token(
    State(state): State<AppState>,
    Json(request): Json<TokenValidationRequest>,
) -> Result<Json<TokenValidationResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/identity/auth/validate", "POST");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    let response = TokenValidationResponse {
        valid: !request.access_token.is_empty(),
        user_id: Some("user_12345".to_string()),
        expires_at: Some(Utc::now() + chrono::Duration::hours(1)),
        scopes: vec!["read".to_string(), "write".to_string()],
    };

    stub.log_response(
        &context,
        "/api/identity/auth/validate",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn get_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<UsersResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/identity/users", "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    let users = vec![User {
        user_id: "user_001".to_string(),
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        wallet_address: if context.enable_crypto {
            Some("0xalice123456789".to_string())
        } else {
            None
        },
        roles: vec!["user".to_string()],
        permissions: vec!["read:profile".to_string()],
        is_active: true,
        created_at: Utc::now() - chrono::Duration::days(30),
        last_login: Some(Utc::now() - chrono::Duration::hours(2)),
    }];

    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100);

    let response = UsersResponse {
        users,
        total_count: 1,
        page,
        limit,
    };

    stub.log_response(
        &context,
        "/api/identity/users",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn get_user_details(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, &format!("/api/identity/users/{}", user_id), "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    let user = User {
        user_id: user_id.clone(),
        username: "mock_user".to_string(),
        email: "mock_user@example.com".to_string(),
        wallet_address: if context.enable_crypto {
            Some("0xmockuser123456789".to_string())
        } else {
            None
        },
        roles: vec!["user".to_string()],
        permissions: vec!["read:profile".to_string()],
        is_active: true,
        created_at: Utc::now() - chrono::Duration::days(10),
        last_login: Some(Utc::now() - chrono::Duration::hours(1)),
    };

    stub.log_response(
        &context,
        &format!("/api/identity/users/{}", user_id),
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(user))
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
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", config.base.port + 1000).parse()?; // gRPC on port 9083
    let state = AppState::new(config.clone());

    info!(
        service_name = %config.base.name,
        version = %config.base.version,
        http_address = %http_addr,
        grpc_address = %grpc_addr,
        enable_crypto = config.dual_mode.enable_crypto,
        "Starting Identity Service Smart Stub with HTTP and gRPC servers"
    );

    let app = Router::new()
        // Health and configuration endpoints
        .route("/health", get(health_check))
        .route("/stub/config", get(get_stub_config))
        .route("/stub/reset", post(reset_stub_state))
        // Authentication endpoints
        .route("/api/identity/auth/login", post(login))
        .route("/api/identity/auth/register", post(register))
        .route("/api/identity/auth/validate", post(validate_token))
        // User management endpoints
        .route("/api/identity/users", get(get_users))
        .route("/api/identity/users/:user_id", get(get_user_details))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let http_listener = tokio::net::TcpListener::bind(http_addr).await?;

    // gRPC Server
    let grpc_service = IdentityGrpcService::new(config.clone());

    info!("HTTP server ready and listening on {}", http_addr);
    info!("gRPC server ready and listening on {}", grpc_addr);

    // Run both servers concurrently
    let http_server = axum::serve(http_listener, app).with_graceful_shutdown(shutdown_signal());

    let grpc_server = Server::builder()
        .add_service(IdentityServiceServer::new(grpc_service))
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
