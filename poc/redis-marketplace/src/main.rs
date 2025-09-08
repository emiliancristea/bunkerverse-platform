// Redis-Integrated Marketplace Service PoC
// Demonstrates Rust/Axum with Redis for real-time NFT marketplace functionality

use axum::{
    extract::{ws::WebSocket, ws::WebSocketUpgrade, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use deadpool_redis::{Config, Runtime};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing::{info, error, debug};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};
use anyhow::Result;

// ===== TESTING MODULE =====
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct NFTListing {
    pub id: String,
    pub nft_contract: String,
    pub token_id: String,
    pub seller: String,
    pub price: String, // In Wei as string to avoid precision loss
    pub currency: String, // ETH, BNK, etc.
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub metadata_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ListingStatus,
    pub category: String,
    pub attributes: HashMap<String, String>,
    pub view_count: u64,
    pub favorite_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingStatus {
    Active,
    Sold,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateListingRequest {
    #[validate(length(min = 42, max = 42))] // Ethereum address length
    pub nft_contract: String,
    pub token_id: String,
    #[validate(length(min = 42, max = 42))]
    pub seller: String,
    pub price: String,
    pub currency: String,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(max = 2000))]
    pub description: String,
    pub image_url: Option<String>,
    pub metadata_url: String,
    pub category: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidRequest {
    pub listing_id: String,
    pub bidder: String,
    pub amount: String,
    pub expiry: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub id: String,
    pub listing_id: String,
    pub bidder: String,
    pub amount: String,
    pub created_at: DateTime<Utc>,
    pub expiry: Option<DateTime<Utc>>,
    pub status: BidStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BidStatus {
    Active,
    Accepted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    pub total_listings: u64,
    pub active_listings: u64,
    pub total_volume: String,
    pub unique_sellers: u64,
    pub unique_buyers: u64,
    pub average_price: String,
    pub top_categories: Vec<(String, u64)>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    NewListing(NFTListing),
    ListingUpdate(NFTListing),
    NewBid(Bid),
    BidUpdate(Bid),
    PriceAlert { listing_id: String, old_price: String, new_price: String },
    MarketplaceStats(MarketplaceStats),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: Option<String>,
    pub category: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
    pub seller: Option<String>,
    pub status: Option<String>,
    pub sort_by: Option<String>, // price, created_at, view_count
    pub sort_order: Option<String>, // asc, desc
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// ===== APPLICATION STATE =====

#[derive(Clone)]
pub struct AppState {
    pub redis_pool: deadpool_redis::Pool,
    pub websocket_tx: broadcast::Sender<WebSocketMessage>,
    pub performance_metrics: Arc<tokio::sync::RwLock<PerformanceMetrics>>,
}

#[derive(Debug)]
pub struct PerformanceMetrics {
    pub requests_total: u64,
    pub requests_per_second: f64,
    pub average_response_time: Duration,
    pub redis_operations: u64,
    pub websocket_connections: u64,
    pub listings_created: u64,
    pub bids_placed: u64,
    pub last_updated: Instant,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            requests_total: 0,
            requests_per_second: 0.0,
            average_response_time: Duration::from_secs(0),
            redis_operations: 0,
            websocket_connections: 0,
            listings_created: 0,
            bids_placed: 0,
            last_updated: Instant::now(),
        }
    }
}

// ===== MARKETPLACE SERVICE IMPLEMENTATION =====

impl AppState {
    pub async fn new(redis_url: &str) -> Result<Self> {
        // Initialize Redis connection pool
        let cfg = Config::from_url(redis_url);
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        
        // Test Redis connection
        let mut conn = pool.get().await?;
        redis::cmd("PING").query_async::<_, String>(&mut conn).await?;
        info!("✅ Redis connection established");

        // Create WebSocket broadcast channel
        let (websocket_tx, _) = broadcast::channel(1000);
        
        let metrics = Arc::new(tokio::sync::RwLock::new(PerformanceMetrics::default()));

        Ok(Self {
            redis_pool: pool,
            websocket_tx,
            performance_metrics: metrics,
        })
    }

    // Create a new NFT listing
    pub async fn create_listing(&self, req: CreateListingRequest) -> Result<NFTListing> {
        let start_time = Instant::now();
        
        let listing = NFTListing {
            id: Uuid::new_v4().to_string(),
            nft_contract: req.nft_contract,
            token_id: req.token_id,
            seller: req.seller,
            price: req.price,
            currency: req.currency,
            title: req.title,
            description: req.description,
            image_url: req.image_url,
            metadata_url: req.metadata_url,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: ListingStatus::Active,
            category: req.category,
            attributes: req.attributes,
            view_count: 0,
            favorite_count: 0,
        };

        // Store in Redis
        let mut conn = self.redis_pool.get().await?;
        let listing_json = serde_json::to_string(&listing)?;
        
        // Store listing with multiple indexes
        let _: () = redis::pipe()
            .hset("listings", &listing.id, &listing_json)
            .zadd("listings_by_price", &listing.price.parse::<f64>().unwrap_or(0.0), &listing.id)
            .zadd("listings_by_date", listing.created_at.timestamp(), &listing.id)
            .sadd(format!("listings_by_category:{}", listing.category), &listing.id)
            .sadd(format!("listings_by_seller:{}", listing.seller), &listing.id)
            .incr("stats:total_listings", 1)
            .incr("stats:active_listings", 1)
            .query_async(&mut conn).await?;

        // Update performance metrics
        self.update_metrics(start_time, "create_listing").await;

        // Broadcast to WebSocket clients
        let _ = self.websocket_tx.send(WebSocketMessage::NewListing(listing.clone()));

        info!("📝 Created listing {} for NFT {}/{}", listing.id, listing.nft_contract, listing.token_id);
        Ok(listing)
    }

    // Get listing by ID
    pub async fn get_listing(&self, listing_id: &str) -> Result<Option<NFTListing>> {
        let start_time = Instant::now();
        
        let mut conn = self.redis_pool.get().await?;
        let listing_json: Option<String> = redis::cmd("HGET")
            .arg("listings")
            .arg(listing_id)
            .query_async(&mut conn).await?;

        if let Some(json) = listing_json {
            // Increment view count
            let _: () = redis::cmd("HINCRBY")
                .arg("listing_views")
                .arg(listing_id)
                .arg(1)
                .query_async(&mut conn).await?;

            let mut listing: NFTListing = serde_json::from_str(&json)?;
            listing.view_count += 1;

            self.update_metrics(start_time, "get_listing").await;
            Ok(Some(listing))
        } else {
            self.update_metrics(start_time, "get_listing").await;
            Ok(None)
        }
    }

    // Search listings with filters
    pub async fn search_listings(&self, query: SearchQuery) -> Result<(Vec<NFTListing>, u64)> {
        let start_time = Instant::now();
        
        let mut conn = self.redis_pool.get().await?;
        let page = query.page.unwrap_or(1);
        let limit = query.limit.unwrap_or(20).min(100); // Cap at 100
        let offset = (page - 1) * limit;

        // Build Redis query based on filters
        let listing_ids: Vec<String> = if let Some(category) = &query.category {
            redis::cmd("SMEMBERS")
                .arg(format!("listings_by_category:{}", category))
                .query_async(&mut conn).await?
        } else if let Some(seller) = &query.seller {
            redis::cmd("SMEMBERS")
                .arg(format!("listings_by_seller:{}", seller))
                .query_async(&mut conn).await?
        } else {
            // Get all active listings sorted by date (newest first)
            redis::cmd("ZREVRANGE")
                .arg("listings_by_date")
                .arg(offset as isize)
                .arg((offset + limit - 1) as isize)
                .query_async(&mut conn).await?
        };

        // Fetch listing details
        let mut listings = Vec::new();
        if !listing_ids.is_empty() {
            let listing_jsons: Vec<String> = redis::cmd("HMGET")
                .arg("listings")
                .arg(&listing_ids)
                .query_async(&mut conn).await?;

            for json in listing_jsons {
                if let Ok(listing) = serde_json::from_str::<NFTListing>(&json) {
                    // Apply additional filters
                    if self.matches_filters(&listing, &query) {
                        listings.push(listing);
                    }
                }
            }
        }

        // Sort results
        self.sort_listings(&mut listings, &query);

        let total_count = listing_ids.len() as u64;
        self.update_metrics(start_time, "search_listings").await;

        info!("🔍 Search returned {} listings (page {}, limit {})", listings.len(), page, limit);
        Ok((listings, total_count))
    }

    // Place a bid on a listing
    pub async fn place_bid(&self, bid_request: BidRequest) -> Result<Bid> {
        let start_time = Instant::now();
        
        let bid = Bid {
            id: Uuid::new_v4().to_string(),
            listing_id: bid_request.listing_id.clone(),
            bidder: bid_request.bidder,
            amount: bid_request.amount,
            created_at: Utc::now(),
            expiry: bid_request.expiry,
            status: BidStatus::Active,
        };

        let mut conn = self.redis_pool.get().await?;
        let bid_json = serde_json::to_string(&bid)?;
        
        // Store bid
        let _: () = redis::pipe()
            .hset("bids", &bid.id, &bid_json)
            .zadd(format!("bids_by_listing:{}", bid.listing_id), bid.amount.parse::<f64>().unwrap_or(0.0), &bid.id)
            .zadd("bids_by_amount", bid.amount.parse::<f64>().unwrap_or(0.0), &bid.id)
            .incr("stats:total_bids", 1)
            .query_async(&mut conn).await?;

        self.update_metrics(start_time, "place_bid").await;

        // Broadcast to WebSocket clients
        let _ = self.websocket_tx.send(WebSocketMessage::NewBid(bid.clone()));

        info!("💰 Placed bid {} on listing {} for {} wei", bid.id, bid.listing_id, bid.amount);
        Ok(bid)
    }

    // Get marketplace statistics
    pub async fn get_marketplace_stats(&self) -> Result<MarketplaceStats> {
        let start_time = Instant::now();
        
        let mut conn = self.redis_pool.get().await?;
        
        // Get basic statistics
        let (total_listings, active_listings): (u64, u64) = redis::pipe()
            .get("stats:total_listings")
            .get("stats:active_listings")
            .query_async(&mut conn).await?;

        // Calculate volume and averages (simplified for PoC)
        let total_volume = "1250000000000000000"; // 1.25 ETH in Wei
        let average_price = "500000000000000000"; // 0.5 ETH in Wei
        
        // Get top categories
        let categories: Vec<String> = redis::cmd("KEYS")
            .arg("listings_by_category:*")
            .query_async(&mut conn).await?;
            
        let mut top_categories = Vec::new();
        for cat_key in categories {
            let count: u64 = redis::cmd("SCARD")
                .arg(&cat_key)
                .query_async(&mut conn).await?;
            let category = cat_key.split(':').last().unwrap_or("unknown");
            top_categories.push((category.to_string(), count));
        }
        
        top_categories.sort_by(|a, b| b.1.cmp(&a.1));
        top_categories.truncate(5);

        let stats = MarketplaceStats {
            total_listings,
            active_listings,
            total_volume: total_volume.to_string(),
            unique_sellers: 42, // Mock data for PoC
            unique_buyers: 28,
            average_price: average_price.to_string(),
            top_categories,
            last_updated: Utc::now(),
        };

        self.update_metrics(start_time, "get_marketplace_stats").await;
        Ok(stats)
    }

    // Helper methods
    fn matches_filters(&self, listing: &NFTListing, query: &SearchQuery) -> bool {
        if let Some(status) = &query.status {
            let status_matches = match status.as_str() {
                "active" => matches!(listing.status, ListingStatus::Active),
                "sold" => matches!(listing.status, ListingStatus::Sold),
                "cancelled" => matches!(listing.status, ListingStatus::Cancelled),
                "expired" => matches!(listing.status, ListingStatus::Expired),
                _ => false,
            };
            if !status_matches {
                return false;
            }
        }

        if let Some(q) = &query.query {
            let q_lower = q.to_lowercase();
            if !listing.title.to_lowercase().contains(&q_lower) 
                && !listing.description.to_lowercase().contains(&q_lower) {
                return false;
            }
        }

        if let Some(min_price) = &query.min_price {
            if let (Ok(min), Ok(current)) = (min_price.parse::<f64>(), listing.price.parse::<f64>()) {
                if current < min {
                    return false;
                }
            }
        }

        if let Some(max_price) = &query.max_price {
            if let (Ok(max), Ok(current)) = (max_price.parse::<f64>(), listing.price.parse::<f64>()) {
                if current > max {
                    return false;
                }
            }
        }

        true
    }

    fn sort_listings(&self, listings: &mut Vec<NFTListing>, query: &SearchQuery) {
        if let Some(sort_by) = &query.sort_by {
            let ascending = query.sort_order.as_deref() == Some("asc");
            
            match sort_by.as_str() {
                "price" => {
                    listings.sort_by(|a, b| {
                        let price_a = a.price.parse::<f64>().unwrap_or(0.0);
                        let price_b = b.price.parse::<f64>().unwrap_or(0.0);
                        if ascending {
                            price_a.partial_cmp(&price_b).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            price_b.partial_cmp(&price_a).unwrap_or(std::cmp::Ordering::Equal)
                        }
                    });
                },
                "created_at" => {
                    listings.sort_by(|a, b| {
                        if ascending {
                            a.created_at.cmp(&b.created_at)
                        } else {
                            b.created_at.cmp(&a.created_at)
                        }
                    });
                },
                "view_count" => {
                    listings.sort_by(|a, b| {
                        if ascending {
                            a.view_count.cmp(&b.view_count)
                        } else {
                            b.view_count.cmp(&a.view_count)
                        }
                    });
                },
                _ => {} // No sorting
            }
        }
    }

    async fn update_metrics(&self, start_time: Instant, operation: &str) {
        let mut metrics = self.performance_metrics.write().await;
        metrics.requests_total += 1;
        
        let duration = start_time.elapsed();
        let total_time = metrics.average_response_time.as_nanos() as f64 * (metrics.requests_total - 1) as f64;
        metrics.average_response_time = Duration::from_nanos(
            ((total_time + duration.as_nanos() as f64) / metrics.requests_total as f64) as u64
        );

        if operation.contains("redis") || operation.contains("listing") || operation.contains("bid") {
            metrics.redis_operations += 1;
        }

        match operation {
            "create_listing" => metrics.listings_created += 1,
            "place_bid" => metrics.bids_placed += 1,
            _ => {}
        }

        metrics.last_updated = Instant::now();
        
        debug!("📊 Updated metrics for {}: {:?}", operation, duration);
    }
}

// ===== HTTP HANDLERS =====

async fn create_listing_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateListingRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match req.validate() {
        Ok(_) => {},
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    }

    match state.create_listing(req).await {
        Ok(listing) => Ok(Json(listing)),
        Err(e) => {
            error!("Failed to create listing: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_listing_handler(
    State(state): State<AppState>,
    Path(listing_id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.get_listing(&listing_id).await {
        Ok(Some(listing)) => Ok(Json(listing)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("Failed to get listing: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn search_listings_handler(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.search_listings(query).await {
        Ok((listings, total)) => Ok(Json(serde_json::json!({
            "listings": listings,
            "total": total,
            "timestamp": Utc::now()
        }))),
        Err(e) => {
            error!("Failed to search listings: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn place_bid_handler(
    State(state): State<AppState>,
    Json(req): Json<BidRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.place_bid(req).await {
        Ok(bid) => Ok(Json(bid)),
        Err(e) => {
            error!("Failed to place bid: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_stats_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.get_marketplace_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(e) => {
            error!("Failed to get marketplace stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "service": "redis-marketplace-poc"
    }))
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.performance_metrics.read().await;
    Json(serde_json::json!({
        "requests_total": metrics.requests_total,
        "average_response_time_ms": metrics.average_response_time.as_millis(),
        "redis_operations": metrics.redis_operations,
        "websocket_connections": metrics.websocket_connections,
        "listings_created": metrics.listings_created,
        "bids_placed": metrics.bids_placed,
        "last_updated": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }))
}

// ===== WEBSOCKET HANDLER =====

async fn websocket_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.websocket_tx.subscribe();

    // Update connection count
    {
        let mut metrics = state.performance_metrics.write().await;
        metrics.websocket_connections += 1;
    }

    // Spawn task to send messages to client
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json_msg = serde_json::to_string(&msg).unwrap_or_default();
            if sender.send(axum::extract::ws::Message::Text(json_msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from client
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(msg) = msg {
                match msg {
                    axum::extract::ws::Message::Text(_text) => {
                        // Handle client messages (e.g., subscribe to specific listings)
                        debug!("Received WebSocket message from client");
                    },
                    axum::extract::ws::Message::Close(_) => break,
                    _ => {}
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Update connection count
    {
        let mut metrics = state.performance_metrics.write().await;
        metrics.websocket_connections = metrics.websocket_connections.saturating_sub(1);
    }

    info!("📡 WebSocket connection closed");
}

// ===== DEMO DATA SEEDER =====

async fn seed_demo_data(state: &AppState) -> Result<()> {
    info!("🌱 Seeding demo marketplace data...");

    // Create sample NFT listings
    let demo_listings = vec![
        CreateListingRequest {
            nft_contract: "0x1234567890123456789012345678901234567890".to_string(),
            token_id: "1".to_string(),
            seller: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            price: "1000000000000000000".to_string(), // 1 ETH
            currency: "ETH".to_string(),
            title: "Cosmic Dragon #001".to_string(),
            description: "A rare cosmic dragon with ethereal properties and legendary power.".to_string(),
            image_url: Some("https://example.com/cosmic-dragon-001.png".to_string()),
            metadata_url: "https://example.com/metadata/1".to_string(),
            category: "Dragons".to_string(),
            attributes: HashMap::from([
                ("Rarity".to_string(), "Legendary".to_string()),
                ("Element".to_string(), "Cosmic".to_string()),
                ("Power".to_string(), "9500".to_string()),
            ]),
        },
        CreateListingRequest {
            nft_contract: "0x2345678901234567890123456789012345678901".to_string(),
            token_id: "42".to_string(),
            seller: "0xbcdefabcdefabcdefabcdefabcdefabcdefabcde".to_string(),
            price: "500000000000000000".to_string(), // 0.5 ETH
            currency: "ETH".to_string(),
            title: "Neon Cityscape Virtual Land".to_string(),
            description: "Premium virtual real estate in the neon district with commercial rights.".to_string(),
            image_url: Some("https://example.com/neon-cityscape-042.png".to_string()),
            metadata_url: "https://example.com/metadata/42".to_string(),
            category: "Virtual Land".to_string(),
            attributes: HashMap::from([
                ("Size".to_string(), "500x500".to_string()),
                ("District".to_string(), "Neon".to_string()),
                ("Commercial".to_string(), "Yes".to_string()),
            ]),
        },
        CreateListingRequest {
            nft_contract: "0x3456789012345678901234567890123456789012".to_string(),
            token_id: "777".to_string(),
            seller: "0xcdefabcdefabcdefabcdefabcdefabcdefabcdef".to_string(),
            price: "250000000000000000".to_string(), // 0.25 ETH
            currency: "ETH".to_string(),
            title: "Quantum Sword of Legends".to_string(),
            description: "An ancient weapon forged in quantum realms, +50 Attack Power.".to_string(),
            image_url: Some("https://example.com/quantum-sword-777.png".to_string()),
            metadata_url: "https://example.com/metadata/777".to_string(),
            category: "Weapons".to_string(),
            attributes: HashMap::from([
                ("Type".to_string(), "Sword".to_string()),
                ("Attack Power".to_string(), "50".to_string()),
                ("Special".to_string(), "Quantum Phase".to_string()),
            ]),
        },
    ];

    for listing_req in demo_listings {
        if let Ok(listing) = state.create_listing(listing_req).await {
            info!("✅ Created demo listing: {}", listing.title);
        }
    }

    Ok(())
}

// ===== MAIN APPLICATION =====

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("redis_marketplace_poc=info,tower_http=debug")
        .init();

    info!("🚀 Starting Redis Marketplace Service PoC...");

    // Initialize application state
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let state = AppState::new(&redis_url).await?;

    // Seed demo data
    seed_demo_data(&state).await?;

    // Build the application router
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>BUNKERVERSE Redis Marketplace PoC</h1><p>WebSocket: <code>ws://localhost:3002/ws</code></p>") }))
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/api/listings", post(create_listing_handler))
        .route("/api/listings/:id", get(get_listing_handler))
        .route("/api/listings/search", get(search_listings_handler))
        .route("/api/bids", post(place_bid_handler))
        .route("/api/stats", get(get_stats_handler))
        .route("/ws", get(websocket_handler))
        .layer(CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    info!("🌐 Redis Marketplace Service listening on http://{}", addr);
    info!("📡 WebSocket endpoint: ws://{}/ws", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}