use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStatus {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub total_events: usize,
    pub last_indexed_block: usize,
    pub status: String,
}

#[derive(Clone)]
pub struct AppState {
    pub mock_data: Arc<IndexStatus>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mock_data: Arc::new(IndexStatus {
                total_blocks: 1000,
                total_transactions: 5000,
                total_events: 2500,
                last_indexed_block: 999,
                status: "active".to_string(),
            }),
        }
    }
}

async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "Indexer Service",
        "mode": "mock",
        "version": "0.1.0"
    }))
}

async fn get_index_status(State(state): State<AppState>) -> Result<Json<IndexStatus>, StatusCode> {
    info!("📊 Fetching index status");

    // In a real implementation, this would query actual blockchain data
    warn!("🚧 Using mock indexer data for PoC");

    Ok(Json(IndexStatus {
        total_blocks: state.mock_data.total_blocks,
        total_transactions: state.mock_data.total_transactions,
        total_events: state.mock_data.total_events,
        last_indexed_block: state.mock_data.last_indexed_block,
        status: state.mock_data.status.clone(),
    }))
}

async fn get_latest_blocks(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    info!("🧱 Fetching latest blocks");

    // Mock block data
    let blocks = serde_json::json!({
        "blocks": [
            {
                "number": 999,
                "hash": "0xabc123...",
                "timestamp": "2025-09-08T14:15:00Z",
                "transactions": 5
            },
            {
                "number": 998,
                "hash": "0xdef456...",
                "timestamp": "2025-09-08T14:14:50Z",
                "transactions": 3
            }
        ]
    });

    Ok(Json(blocks))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Bunkerverse Indexer Service (PoC)");

    let state = AppState::new();

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/index/status", get(get_index_status))
        .route("/index/blocks/latest", get(get_latest_blocks))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await?;
    info!("🌐 Indexer service listening on http://0.0.0.0:3003");

    axum::serve(listener, app).await?;

    Ok(())
}
