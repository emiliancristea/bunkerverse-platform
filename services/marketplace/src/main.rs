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
    bunkerverse::services::v1::marketplace_service_server::MarketplaceServiceServer,
    MarketplaceGrpcService,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use stub::{MarketplaceStub, RequestContext, SmartStub};
use tokio::signal;
use tonic::transport::Server;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

// API Request/Response Types
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MarketplaceFilters {
    pub category: Option<String>,
    pub min_price: Option<u64>,
    pub max_price: Option<u64>,
    pub seller: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketListing {
    pub listing_id: String,
    pub nft_id: String,
    pub seller_address: String,
    pub price_wei: String,
    pub currency: String,
    pub listing_type: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct MarketListingsResponse {
    pub listings: Vec<MarketListing>,
    pub total_count: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftDetails {
    pub nft_id: String,
    pub token_id: String,
    pub contract_address: Option<String>,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub owner_address: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateListingRequest {
    pub nft_id: String,
    pub price_wei: String,
    pub currency: String,
    pub listing_type: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CreateListingResponse {
    pub listing_id: String,
    pub transaction_hash: Option<String>,
    pub estimated_gas_fee: Option<String>,
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
    pub stub: Arc<tokio::sync::Mutex<MarketplaceStub>>,
}

impl AppState {
    pub fn new(config: StubConfiguration) -> Self {
        Self {
            stub: Arc::new(tokio::sync::Mutex::new(MarketplaceStub::new(config))),
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

pub async fn get_market_listings(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
    Query(_filters): Query<MarketplaceFilters>,
) -> Result<Json<MarketListingsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/marketplace/listings", "GET");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if stub.should_inject_error_response() {
        stub.log_response(
            &context,
            "/api/marketplace/listings",
            latency.as_millis() as u64,
            500,
            true,
        );
        let error = ErrorResponse {
            error: "Simulated error".to_string(),
            code: "INTERNAL_ERROR".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)));
    }

    // Generate mock listings based on dual-mode configuration
    let listings = if context.enable_crypto {
        // Full crypto mode - return NFT marketplace listings
        vec![
            MarketListing {
                listing_id: "listing_001".to_string(),
                nft_id: "nft_001".to_string(),
                seller_address: "0x1234567890abcdef".to_string(),
                price_wei: "1000000000000000000".to_string(), // 1 ETH
                currency: "ETH".to_string(),
                listing_type: "auction".to_string(),
                created_at: Utc::now(),
                expires_at: Some(Utc::now() + chrono::Duration::days(1)),
                status: "active".to_string(),
            },
            MarketListing {
                listing_id: "listing_002".to_string(),
                nft_id: "nft_002".to_string(),
                seller_address: "0xabcdef1234567890".to_string(),
                price_wei: "500000000000000000".to_string(), // 0.5 ETH
                currency: "ETH".to_string(),
                listing_type: "fixed_price".to_string(),
                created_at: Utc::now() - chrono::Duration::hours(2),
                expires_at: None,
                status: "active".to_string(),
            },
        ]
    } else {
        // MVE mode - return non-crypto marketplace items
        vec![MarketListing {
            listing_id: "listing_001".to_string(),
            nft_id: "item_001".to_string(),
            seller_address: "player_123".to_string(),
            price_wei: "100".to_string(),
            currency: "CREDITS".to_string(),
            listing_type: "fixed_price".to_string(),
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::days(7)),
            status: "active".to_string(),
        }]
    };

    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100); // Cap at 100

    let response = MarketListingsResponse {
        listings,
        total_count: if context.enable_crypto { 2 } else { 1 },
        page,
        limit,
    };

    stub.log_response(
        &context,
        "/api/marketplace/listings",
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(response))
}

pub async fn get_listing_details(
    State(state): State<AppState>,
    Path(listing_id): Path<String>,
) -> Result<Json<MarketListing>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(
        &context,
        &format!("/api/marketplace/listings/{}", listing_id),
        "GET",
    );

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if stub.should_inject_error_response() {
        stub.log_response(
            &context,
            &format!("/api/marketplace/listings/{}", listing_id),
            latency.as_millis() as u64,
            404,
            true,
        );
        let error = ErrorResponse {
            error: "Listing not found".to_string(),
            code: "NOT_FOUND".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::NOT_FOUND, Json(error)));
    }

    let listing = MarketListing {
        listing_id: listing_id.clone(),
        nft_id: "nft_001".to_string(),
        seller_address: if context.enable_crypto {
            "0x1234567890abcdef".to_string()
        } else {
            "player_123".to_string()
        },
        price_wei: if context.enable_crypto {
            "1000000000000000000".to_string()
        } else {
            "100".to_string()
        },
        currency: if context.enable_crypto {
            "ETH"
        } else {
            "CREDITS"
        }
        .to_string(),
        listing_type: "fixed_price".to_string(),
        created_at: Utc::now() - chrono::Duration::hours(1),
        expires_at: Some(Utc::now() + chrono::Duration::days(1)),
        status: "active".to_string(),
    };

    stub.log_response(
        &context,
        &format!("/api/marketplace/listings/{}", listing_id),
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(listing))
}

pub async fn get_nft_details(
    State(state): State<AppState>,
    Path(nft_id): Path<String>,
) -> Result<Json<NftDetails>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(
        &context,
        &format!("/api/marketplace/nfts/{}", nft_id),
        "GET",
    );

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    // Check crypto features
    if !context.enable_crypto {
        stub.log_response(
            &context,
            &format!("/api/marketplace/nfts/{}", nft_id),
            latency.as_millis() as u64,
            403,
            false,
        );
        let error = ErrorResponse {
            error: "NFT features not enabled".to_string(),
            code: "FEATURE_NOT_ENABLED".to_string(),
            timestamp: Utc::now(),
            request_id: context.request_id,
        };
        return Err((StatusCode::FORBIDDEN, Json(error)));
    }

    let nft = NftDetails {
        nft_id: nft_id.clone(),
        token_id: "1".to_string(),
        contract_address: Some("0xabcdef1234567890".to_string()),
        name: "Mock NFT".to_string(),
        description: "A mock NFT for testing".to_string(),
        image_url: "https://example.com/nft.png".to_string(),
        owner_address: "0x1234567890abcdef".to_string(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("rarity".to_string(), "epic".to_string());
            meta.insert("level".to_string(), "5".to_string());
            meta
        },
    };

    stub.log_response(
        &context,
        &format!("/api/marketplace/nfts/{}", nft_id),
        latency.as_millis() as u64,
        200,
        false,
    );
    Ok(Json(nft))
}

pub async fn create_listing(
    State(state): State<AppState>,
    Json(_request): Json<CreateListingRequest>,
) -> Result<Json<CreateListingResponse>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(&context, "/api/marketplace/listings", "POST");

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    // Check crypto features for blockchain operations
    if let Err(err) = stub.check_crypto_features(&context) {
        stub.log_response(
            &context,
            "/api/marketplace/listings",
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

    let response = CreateListingResponse {
        listing_id: Uuid::new_v4().to_string(),
        transaction_hash: if context.enable_crypto {
            Some("0xmocktxhash12345".to_string())
        } else {
            None
        },
        estimated_gas_fee: if context.enable_crypto {
            Some("21000".to_string())
        } else {
            None
        },
    };

    stub.log_response(
        &context,
        "/api/marketplace/listings",
        latency.as_millis() as u64,
        201,
        false,
    );
    Ok(Json(response))
}

pub async fn get_player_nfts(
    State(state): State<AppState>,
    Path(player_address): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let context = state.create_context(None).await;
    let stub = state.stub.lock().await;

    stub.log_request(
        &context,
        &format!("/api/marketplace/players/{}/nfts", player_address),
        "GET",
    );

    // Simulate latency
    let latency = stub.calculate_response_latency();
    tokio::time::sleep(tokio::time::Duration::from_millis(
        latency.as_millis() as u64
    ))
    .await;

    if !context.enable_crypto {
        // Return empty array for MVE mode
        let response = serde_json::json!({
            "nfts": [],
            "total_count": 0,
            "page": pagination.page.unwrap_or(1),
            "limit": pagination.limit.unwrap_or(10)
        });
        stub.log_response(
            &context,
            &format!("/api/marketplace/players/{}/nfts", player_address),
            latency.as_millis() as u64,
            200,
            false,
        );
        return Ok(Json(response));
    }

    // Return mock NFTs for crypto mode
    let nfts = vec![NftDetails {
        nft_id: "nft_player_001".to_string(),
        token_id: "42".to_string(),
        contract_address: Some("0xabcdef1234567890".to_string()),
        name: "Player's Epic Weapon".to_string(),
        description: "A legendary weapon NFT owned by the player".to_string(),
        image_url: "https://example.com/player-weapon.png".to_string(),
        owner_address: player_address.clone(),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("type".to_string(), "weapon".to_string());
            meta.insert("rarity".to_string(), "epic".to_string());
            meta.insert("damage".to_string(), "150".to_string());
            meta
        },
    }];

    let response = serde_json::json!({
        "nfts": nfts,
        "total_count": 1,
        "page": pagination.page.unwrap_or(1),
        "limit": pagination.limit.unwrap_or(10)
    });

    stub.log_response(
        &context,
        &format!("/api/marketplace/players/{}/nfts", player_address),
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
    let grpc_addr: SocketAddr = format!("0.0.0.0:{}", config.base.port + 1000).parse()?; // gRPC on port 5080
    let state = AppState::new(config.clone());

    info!(
        service_name = %config.base.name,
        version = %config.base.version,
        http_address = %http_addr,
        grpc_address = %grpc_addr,
        enable_crypto = config.dual_mode.enable_crypto,
        "Starting Marketplace Service Smart Stub with HTTP and gRPC servers"
    );

    // HTTP Server
    let app = Router::new()
        // Health and configuration endpoints
        .route("/health", get(health_check))
        .route("/stub/config", get(get_stub_config))
        .route("/stub/reset", post(reset_stub_state))
        // Marketplace API endpoints
        .route("/api/marketplace/listings", get(get_market_listings))
        .route("/api/marketplace/listings", post(create_listing))
        .route(
            "/api/marketplace/listings/:listing_id",
            get(get_listing_details),
        )
        .route("/api/marketplace/nfts/:nft_id", get(get_nft_details))
        .route(
            "/api/marketplace/players/:player_address/nfts",
            get(get_player_nfts),
        )
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let http_listener = tokio::net::TcpListener::bind(http_addr).await?;

    // gRPC Server
    let grpc_service = MarketplaceGrpcService::new(config.clone());

    info!("HTTP server ready and listening on {}", http_addr);
    info!("gRPC server ready and listening on {}", grpc_addr);

    // Run both servers concurrently
    let http_server = axum::serve(http_listener, app).with_graceful_shutdown(shutdown_signal());

    let grpc_server = Server::builder()
        .add_service(MarketplaceServiceServer::new(grpc_service))
        .serve_with_shutdown(grpc_addr, shutdown_signal());

    // Use tokio::try_join to run both servers concurrently
    let http_result = http_server;
    let grpc_result = grpc_server;

    tokio::select! {
        result = http_result => {
            if let Err(e) = result {
                return Err(anyhow::anyhow!("HTTP server error: {}", e));
            }
        },
        result = grpc_result => {
            if let Err(e) = result {
                return Err(anyhow::anyhow!("gRPC server error: {}", e));
            }
        }
    }

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
