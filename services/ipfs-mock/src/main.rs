mod cid;
mod content;

use anyhow::Result;
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use content::ContentGenerator;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::signal;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Configuration
#[derive(Debug, Clone)]
pub struct IpfsConfig {
    pub port: u16,
    pub service_name: String,
    pub version: String,
    pub enable_detailed_logging: bool,
}

impl Default for IpfsConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            service_name: "IPFS Mock Gateway".to_string(),
            version: "1.0.0".to_string(),
            enable_detailed_logging: std::env::var("RUST_LOG")
                .unwrap_or_default()
                .contains("debug"),
        }
    }
}

// API Response Types
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub service_name: String,
    pub version: String,
    pub gateway_version: String,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct IpfsApiResponse {
    pub hash: String,
    pub name: String,
    pub size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ApiV0Query {
    pub arg: Option<String>,
    pub format: Option<String>,
    pub offline: Option<bool>,
}

// Application State
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<IpfsConfig>,
    pub content_generator: Arc<ContentGenerator>,
}

impl AppState {
    pub fn new(config: IpfsConfig) -> Self {
        Self {
            config: Arc::new(config),
            content_generator: Arc::new(ContentGenerator::new()),
        }
    }
}

// Content Response with proper headers
pub struct IpfsContentResponse {
    pub content: Bytes,
    pub content_type: String,
    pub headers: HeaderMap,
}

impl IntoResponse for IpfsContentResponse {
    fn into_response(self) -> Response {
        let mut response = self.content.into_response();

        // Set content type
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            self.content_type
                .parse()
                .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
        );

        // Add IPFS-specific headers
        response.headers_mut().insert(
            "x-ipfs-path",
            format!("/ipfs/{}", "mock-cid").parse().unwrap(),
        );

        response
            .headers_mut()
            .insert("x-ipfs-gateway-host", "ipfs-mock".parse().unwrap());

        // Add any additional headers
        for (key, value) in self.headers.iter() {
            response.headers_mut().insert(key, value.clone());
        }

        response
    }
}

// Handler Functions
pub async fn health_check(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    let mut details = HashMap::new();
    details.insert("gateway_ready".to_string(), "true".to_string());
    details.insert(
        "content_types_supported".to_string(),
        "json,text,binary,image".to_string(),
    );
    details.insert("api_version".to_string(), "v0".to_string());

    let response = HealthResponse {
        status: "HEALTHY".to_string(),
        timestamp: Utc::now(),
        service_name: state.config.service_name.clone(),
        version: state.config.version.clone(),
        gateway_version: "0.14.0".to_string(), // Mock IPFS gateway version
        details,
    };

    Ok(Json(response))
}

pub async fn get_ipfs_content(
    State(state): State<AppState>,
    Path(cid): Path<String>,
) -> Result<IpfsContentResponse, (StatusCode, Json<ErrorResponse>)> {
    info!(cid = %cid, "IPFS Gateway: Fetching content");

    // Validate CID format (basic validation)
    if cid.is_empty() || cid.len() < 10 {
        warn!(cid = %cid, "Invalid CID format");
        let error = ErrorResponse {
            error: "Invalid CID format".to_string(),
            code: "INVALID_CID".to_string(),
            timestamp: Utc::now(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error)));
    }

    // Generate mock content based on CID
    let (content, content_type) = state.content_generator.generate_content(&cid);

    let mut headers = HeaderMap::new();
    headers.insert(
        "cache-control",
        "public, max-age=29030400, immutable".parse().unwrap(),
    );
    headers.insert("etag", format!("\"{}\"", cid).parse().unwrap());

    info!(
        cid = %cid,
        content_type = %content_type,
        content_size = content.len(),
        "IPFS Gateway: Content served"
    );

    Ok(IpfsContentResponse {
        content: Bytes::from(content),
        content_type,
        headers,
    })
}

pub async fn head_ipfs_content(
    State(state): State<AppState>,
    Path(cid): Path<String>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    info!(cid = %cid, "IPFS Gateway: HEAD request for content metadata");

    // Validate CID format
    if cid.is_empty() || cid.len() < 10 {
        let error = ErrorResponse {
            error: "Invalid CID format".to_string(),
            code: "INVALID_CID".to_string(),
            timestamp: Utc::now(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error)));
    }

    // Generate mock content to get metadata
    let (content, content_type) = state.content_generator.generate_content(&cid);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, &content_type)
        .header(header::CONTENT_LENGTH, content.len())
        .header("cache-control", "public, max-age=29030400, immutable")
        .header("etag", format!("\"{}\"", cid))
        .header("x-ipfs-path", format!("/ipfs/{}", cid))
        .header("x-ipfs-gateway-host", "ipfs-mock")
        .body(axum::body::Body::empty())
        .unwrap();

    info!(
        cid = %cid,
        content_type = %content_type,
        content_size = content.len(),
        "IPFS Gateway: HEAD metadata served"
    );

    Ok(response)
}

// IPFS API v0 endpoints simulation
pub async fn api_v0_cat(
    State(state): State<AppState>,
    Query(params): Query<ApiV0Query>,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    let cid = params.arg.unwrap_or_default();

    info!(cid = %cid, "IPFS API: cat command");

    if cid.is_empty() {
        let error = ErrorResponse {
            error: "Missing required argument: hash".to_string(),
            code: "MISSING_ARG".to_string(),
            timestamp: Utc::now(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error)));
    }

    let (content, content_type) = state.content_generator.generate_content(&cid);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header("x-ipfs-path", format!("/ipfs/{}", cid))
        .body(axum::body::Body::from(content))
        .unwrap();

    Ok(response)
}

pub async fn api_v0_ls(
    State(_state): State<AppState>,
    Query(params): Query<ApiV0Query>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let cid = params.arg.unwrap_or_default();

    info!(cid = %cid, "IPFS API: ls command");

    if cid.is_empty() {
        let error = ErrorResponse {
            error: "Missing required argument: hash".to_string(),
            code: "MISSING_ARG".to_string(),
            timestamp: Utc::now(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error)));
    }

    // Mock directory listing response
    let response = serde_json::json!({
        "Objects": [{
            "Hash": cid,
            "Links": [
                {
                    "Name": "metadata.json",
                    "Hash": format!("Qm{}", cid.chars().skip(2).collect::<String>()),
                    "Size": 256,
                    "Type": 2
                },
                {
                    "Name": "image.png",
                    "Hash": format!("Qm{}", cid.chars().rev().take(44).collect::<String>()),
                    "Size": 1024000,
                    "Type": 2
                }
            ]
        }]
    });

    Ok(Json(response))
}

pub async fn api_v0_add(
    State(_state): State<AppState>,
) -> Result<Json<IpfsApiResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("IPFS API: add command (mock)");

    // Generate a mock hash for uploaded content
    let mock_hash = format!(
        "QmT{}",
        uuid::Uuid::new_v4()
            .to_string()
            .replace("-", "")
            .chars()
            .take(44)
            .collect::<String>()
    );

    let response = IpfsApiResponse {
        hash: mock_hash,
        name: "uploaded-file".to_string(),
        size: Some(1024),
    };

    Ok(Json(response))
}

pub async fn api_v0_id(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("IPFS API: id command");

    let response = serde_json::json!({
        "ID": "12D3KooWMockNodeID1234567890abcdef",
        "PublicKey": "CAASogEwgZ8wDQYJKoZIhvcNAQEBBQADgY0AMIGJAoGBAMock",
        "Addresses": [
            "/ip4/127.0.0.1/tcp/4001",
            "/ip4/172.20.0.5/tcp/4001"
        ],
        "AgentVersion": format!("ipfs-mock/{}", state.config.version),
        "ProtocolVersion": "ipfs/0.1.0"
    });

    Ok(Json(response))
}

pub async fn api_v0_version(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("IPFS API: version command");

    let response = serde_json::json!({
        "Version": "0.14.0-mock",
        "Commit": "mock-commit-hash",
        "Repo": "11",
        "System": "amd64/linux",
        "Golang": "go1.19.1",
        "Service": state.config.service_name
    });

    Ok(Json(response))
}

// Health check function for Docker health checks
async fn perform_health_check() -> Result<()> {
    // Simple health check - just verify the binary can run
    println!("IPFS Mock Gateway health check: OK");
    std::process::exit(0);
}

#[tokio::main]
async fn main() -> Result<()> {
    // Check for health check argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "--health-check" {
        return perform_health_check().await;
    }

    // Initialize tracing with structured JSON logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    let config = IpfsConfig::default();
    let addr: SocketAddr = format!("0.0.0.0:{}", config.port).parse()?;
    let state = AppState::new(config.clone());

    info!(
        service_name = %config.service_name,
        version = %config.version,
        address = %addr,
        gateway_ready = true,
        "Starting IPFS Mock Gateway"
    );

    let app = Router::new()
        // Health endpoint
        .route("/health", get(health_check))
        // IPFS Gateway endpoints
        .route("/ipfs/:cid", get(get_ipfs_content))
        .route("/ipfs/:cid", axum::routing::head(head_ipfs_content))
        // IPFS API v0 endpoints
        .route("/api/v0/cat", get(api_v0_cat))
        .route("/api/v0/cat", post(api_v0_cat))
        .route("/api/v0/ls", get(api_v0_ls))
        .route("/api/v0/ls", post(api_v0_ls))
        .route("/api/v0/add", post(api_v0_add))
        .route("/api/v0/id", get(api_v0_id))
        .route("/api/v0/id", post(api_v0_id))
        .route("/api/v0/version", get(api_v0_version))
        .route("/api/v0/version", post(api_v0_version))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("IPFS Mock Gateway ready and listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

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
