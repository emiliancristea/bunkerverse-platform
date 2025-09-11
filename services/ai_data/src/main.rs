use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use nar_rust_wrapper::{GenerationRequest, GenerationResponse, NarConfig, NarEngine};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{error, info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct AiRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub context: Option<String>, // Additional context for the AI
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    pub response: String,
    pub tokens_used: usize,
    pub processing_time_ms: u64,
    pub model_info: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfoResponse {
    pub model_path: String,
    pub context_size: i32,
    pub embedding_size: i32,
    pub vocabulary_size: i32,
    pub gpu_layers: i32,
    pub threads: i32,
    pub status: String,
}

// Application state
#[derive(Clone)]
pub struct AppState {
    pub nar_engine: Arc<RwLock<Option<NarEngine>>>,
    pub config: NarConfig,
}

impl AppState {
    pub fn new(config: NarConfig) -> Self {
        Self {
            nar_engine: Arc::new(RwLock::new(None)),
            config,
        }
    }

    pub async fn initialize_engine(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🤖 Initializing NAR Engine...");

        // For PoC, we'll simulate engine initialization
        // In a real implementation, this would load the actual model
        match NarEngine::new(self.config.clone()) {
            Ok(engine) => {
                info!("✅ NAR Engine initialized successfully");
                let mut engine_lock = self.nar_engine.write().await;
                *engine_lock = Some(engine);
                Ok(())
            }
            Err(e) => {
                error!("❌ Failed to initialize NAR Engine: {}", e);
                // For PoC, we'll continue without the engine
                warn!("🚧 Running in mock mode without actual NAR engine");
                Ok(())
            }
        }
    }
}

// Handlers
async fn health_check(State(state): State<AppState>) -> Json<serde_json::Value> {
    let engine_status = {
        let engine = state.nar_engine.read().await;
        if engine.is_some() {
            "ready"
        } else {
            "mock_mode"
        }
    };

    Json(serde_json::json!({
        "status": "healthy",
        "service": "AI Data Service",
        "nar_engine": engine_status,
        "model_path": state.config.model_path
    }))
}

async fn get_model_info(
    State(state): State<AppState>,
) -> Result<Json<ModelInfoResponse>, StatusCode> {
    let engine = state.nar_engine.read().await;

    if let Some(ref engine) = *engine {
        match engine.get_model_info() {
            Ok(info) => Ok(Json(ModelInfoResponse {
                model_path: state.config.model_path.clone(),
                context_size: info["context_size"].as_i64().unwrap_or(0) as i32,
                embedding_size: info["embedding_size"].as_i64().unwrap_or(0) as i32,
                vocabulary_size: info["vocabulary_size"].as_i64().unwrap_or(0) as i32,
                gpu_layers: state.config.gpu_layers,
                threads: state.config.threads,
                status: "active".to_string(),
            })),
            Err(e) => {
                error!("Failed to get model info: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        // Mock response for PoC
        Ok(Json(ModelInfoResponse {
            model_path: state.config.model_path.clone(),
            context_size: 2048,
            embedding_size: 4096,
            vocabulary_size: 32000,
            gpu_layers: state.config.gpu_layers,
            threads: state.config.threads,
            status: "mock".to_string(),
        }))
    }
}

async fn generate_ai_response(
    State(state): State<AppState>,
    Json(payload): Json<AiRequest>,
) -> Result<Json<AiResponse>, StatusCode> {
    info!("🧠 AI generation request: {}", payload.prompt);

    let engine = state.nar_engine.read().await;

    if let Some(ref engine) = *engine {
        // Real NAR engine processing
        let mut full_prompt = payload.prompt.clone();
        if let Some(context) = payload.context {
            full_prompt = format!("Context: {}\n\nPrompt: {}", context, full_prompt);
        }

        let request = GenerationRequest {
            prompt: full_prompt,
            max_tokens: payload.max_tokens.unwrap_or(100),
            temperature: payload.temperature.unwrap_or(0.7),
            top_p: 0.9,
            stop_sequences: vec!["<stop>".to_string(), "\n\n".to_string()],
        };

        match engine.generate(request) {
            Ok(response) => {
                let model_info = engine
                    .get_model_info()
                    .unwrap_or_else(|_| serde_json::json!({}));

                info!(
                    "✅ AI generation completed: {} tokens in {}ms",
                    response.tokens_used, response.processing_time_ms
                );

                Ok(Json(AiResponse {
                    response: response.generated_text,
                    tokens_used: response.tokens_used,
                    processing_time_ms: response.processing_time_ms,
                    model_info,
                }))
            }
            Err(e) => {
                error!("AI generation failed: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        // Mock response for PoC
        info!("🚧 Using mock AI response");

        let start_time = std::time::Instant::now();

        // Simulate processing delay
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mock_response = format!(
            "Mock AI Response to: '{}'. This would be generated by NAR/llama.cpp in a real implementation.",
            payload.prompt
        );

        let processing_time = start_time.elapsed().as_millis() as u64;

        Ok(Json(AiResponse {
            response: mock_response,
            tokens_used: 25, // Mock token count
            processing_time_ms: processing_time,
            model_info: serde_json::json!({
                "status": "mock",
                "model": "Gemma-3-1B (simulated)"
            }),
        }))
    }
}

async fn validate_engine(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let engine = state.nar_engine.read().await;

    if let Some(ref engine) = *engine {
        match engine.validate() {
            Ok(()) => Ok(Json(serde_json::json!({
                "status": "validation_passed",
                "message": "NAR Engine is functioning correctly"
            }))),
            Err(e) => {
                error!("Engine validation failed: {}", e);
                Ok(Json(serde_json::json!({
                    "status": "validation_failed",
                    "error": e.to_string()
                })))
            }
        }
    } else {
        Ok(Json(serde_json::json!({
            "status": "mock_mode",
            "message": "Running without actual NAR engine - validation simulated"
        })))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting Bunkerverse AI Data Service (NAR Integration PoC)");

    // Load configuration
    let config = NarConfig {
        model_path: std::env::var("MODEL_PATH")
            .unwrap_or_else(|_| "models/gemma-3-1b.gguf".to_string()),
        context_size: 2048,
        gpu_layers: 0, // CPU only for PoC
        threads: std::env::var("NAR_THREADS")
            .unwrap_or_else(|_| "4".to_string())
            .parse()
            .unwrap_or(4),
        batch_size: 512,
        seed: 42,
    };

    info!(
        "🔧 Configuration loaded: {}",
        serde_json::to_string(&config)?
    );

    let state = AppState::new(config);

    // Initialize NAR engine in background
    let init_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = init_state.initialize_engine().await {
            error!("Failed to initialize NAR engine: {}", e);
        }
    });

    // Build the router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/model/info", get(get_model_info))
        .route("/ai/generate", post(generate_ai_response))
        .route("/ai/validate", get(validate_engine))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await?;
    info!("🌐 AI Data service listening on http://0.0.0.0:3004");

    axum::serve(listener, app).await?;

    Ok(())
}
