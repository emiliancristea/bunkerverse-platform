use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};
use uuid::Uuid;

// Types for zkLogin simulation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub provider: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub nonce: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub provider: String,
    pub id_token: String, // JWT from OAuth provider (simulated)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub user: User,
    pub zkproof: Option<String>, // Simulated ZK proof
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonceRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonceResponse {
    pub nonce: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

// Application state
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<RwLock<HashMap<Uuid, User>>>,
    pub sessions: Arc<RwLock<HashMap<String, Uuid>>>, // token -> user_id
    pub nonces: Arc<RwLock<HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>>, // email -> (nonce, expires)
    pub enable_crypto: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            nonces: Arc::new(RwLock::new(HashMap::new())),
            enable_crypto: std::env::var("ENABLE_CRYPTO").unwrap_or_default() == "true",
        }
    }
}

// Handlers
async fn health_check() -> &'static str {
    "Identity Service is healthy"
}

async fn generate_nonce(
    State(state): State<AppState>,
    Json(payload): Json<NonceRequest>,
) -> Result<Json<NonceResponse>, StatusCode> {
    let nonce = generate_random_nonce();
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(10);

    {
        let mut nonces = state.nonces.write().await;
        nonces.insert(payload.email.clone(), (nonce.clone(), expires_at));
    }

    info!("Generated nonce for email: {}", payload.email);

    Ok(Json(NonceResponse { nonce, expires_at }))
}

async fn zklogin(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    info!(
        "zkLogin attempt for email: {} with provider: {}",
        payload.email, payload.provider
    );

    // In a real implementation, we would:
    // 1. Verify the JWT token with the provider
    // 2. Generate ZK proof of identity
    // 3. Validate the proof against the nonce

    // For PoC: simulate the verification
    let nonce = {
        let nonces = state.nonces.read().await;
        nonces
            .get(&payload.email)
            .map(|(n, expires)| {
                if expires > &chrono::Utc::now() {
                    n.clone()
                } else {
                    return String::new();
                }
            })
            .unwrap_or_default()
    };

    if nonce.is_empty() {
        warn!("No valid nonce found for email: {}", payload.email);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create or get user
    let user = {
        let mut users = state.users.write().await;
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            email: payload.email.clone(),
            provider: payload.provider.clone(),
            created_at: chrono::Utc::now(),
            nonce: nonce.clone(),
        };
        users.insert(user_id, user.clone());
        user
    };

    // Generate access token
    let access_token = generate_access_token(&user);

    // Store session
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(access_token.clone(), user.id);
    }

    // Generate simulated ZK proof (in real implementation this would be cryptographic)
    let zkproof = if state.enable_crypto {
        Some(generate_simulated_zkproof(&user, &nonce))
    } else {
        None
    };

    info!("Successfully created session for user: {}", user.id);

    Ok(Json(LoginResponse {
        access_token,
        user,
        zkproof,
    }))
}

async fn verify_token(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, StatusCode> {
    let sessions = state.sessions.read().await;
    let users = state.users.read().await;

    if let Some(user_id) = sessions.get(&payload.access_token) {
        if let Some(user) = users.get(user_id) {
            info!("Token verified for user: {}", user.id);
            return Ok(Json(VerifyResponse {
                valid: true,
                user: Some(user.clone()),
            }));
        }
    }

    warn!("Invalid token provided");
    Ok(Json(VerifyResponse {
        valid: false,
        user: None,
    }))
}

async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<VerifyRequest>,
) -> Result<StatusCode, StatusCode> {
    let mut sessions = state.sessions.write().await;

    if sessions.remove(&payload.access_token).is_some() {
        info!("User logged out successfully");
        Ok(StatusCode::NO_CONTENT)
    } else {
        warn!("Attempted to logout with invalid token");
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn get_user_info(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<User>, StatusCode> {
    let token = params.get("access_token").ok_or(StatusCode::BAD_REQUEST)?;

    let sessions = state.sessions.read().await;
    let users = state.users.read().await;

    if let Some(user_id) = sessions.get(token) {
        if let Some(user) = users.get(user_id) {
            return Ok(Json(user.clone()));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Utility functions
fn generate_random_nonce() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| rng.gen_range(0..16))
        .map(|n| format!("{:x}", n))
        .collect()
}

fn generate_access_token(user: &User) -> String {
    // In a real implementation, this would be a proper JWT
    format!("bunkerverse_{}_{}", user.id, chrono::Utc::now().timestamp())
}

fn generate_simulated_zkproof(user: &User, nonce: &str) -> String {
    // Simulated ZK proof - in reality this would be a complex cryptographic proof
    use base64::{engine::general_purpose, Engine as _};
    let proof_data = format!("zkproof:{}:{}:{}", user.email, nonce, user.created_at);
    general_purpose::STANDARD.encode(proof_data.as_bytes())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Bunkerverse Identity Service (zkLogin PoC)");

    let state = AppState::new();

    info!("🔐 Crypto features enabled: {}", state.enable_crypto);

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/auth/nonce", post(generate_nonce))
        .route("/auth/zklogin", post(zklogin))
        .route("/auth/verify", post(verify_token))
        .route("/auth/logout", post(logout))
        .route("/user/info", get(get_user_info))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    info!("🌐 Identity service listening on http://0.0.0.0:3001");

    axum::serve(listener, app).await?;

    Ok(())
}
