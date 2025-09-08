use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
mod mock_nar;
use mock_nar::{NarConfig, MockNarEngine as NarEngine, GenerationRequest};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{info, error};
use uuid::Uuid;

// NAR service request/response types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NarServiceRequest {
    pub id: String,
    pub user_id: String,
    pub request_type: NarRequestType,
    pub prompt: String,
    pub parameters: NarParameters,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NarRequestType {
    TextGeneration,
    QuestionAnswering,
    CodeGeneration,
    GameNarrative,
    ChatCompletion,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NarParameters {
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub stop_sequences: Vec<String>,
    pub context_window: Option<usize>,
}

impl Default for NarParameters {
    fn default() -> Self {
        Self {
            max_tokens: 256,
            temperature: 0.7,
            top_p: 0.9,
            stop_sequences: vec![],
            context_window: Some(2048),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NarServiceResponse {
    pub id: String,
    pub request_id: String,
    pub generated_text: String,
    pub tokens_used: usize,
    pub processing_time_ms: u64,
    pub model_info: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub status: ResponseStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseStatus {
    Success,
    PartialSuccess,
    Error(String),
    Timeout,
}

/// NAR Service wrapper for Bunkerverse Platform
pub struct NarService {
    engine: NarEngine,
    model_path: String,
    config: NarConfig,
    request_count: u64,
    total_tokens_generated: u64,
    total_processing_time: Duration,
}

impl NarService {
    /// Create new NAR service with Gemma3-1B model
    pub async fn new() -> Result<Self> {
        info!("🚀 Initializing NAR Service with Gemma3-1B model...");
        
        // Configure for Gemma3-1B model
        let model_path = "D:/code-dev/main/bunkercorporation/bunkerverse-platform/libs/nar-rust-wrapper-for-llama-cpp/models/gemma-3-1b-it-BF16.gguf";
        let config = NarConfig {
            model_path: model_path.to_string(),
            context_size: 2048,
            gpu_layers: 0, // CPU-only for PoC
            threads: 4,
            batch_size: 512,
            seed: 42,
        };

        info!("📖 Loading model from: {}", model_path);
        
        let engine = NarEngine::new(config.clone())
            .context("Failed to initialize NAR engine")?;

        info!("✅ NAR engine initialized successfully");
        
        let service = Self {
            engine,
            model_path: model_path.to_string(),
            config,
            request_count: 0,
            total_tokens_generated: 0,
            total_processing_time: Duration::new(0, 0),
        };

        // Validate the engine
        let mut service = service;
        service.validate_engine().await?;
        
        Ok(service)
    }

    /// Validate engine functionality
    async fn validate_engine(&mut self) -> Result<()> {
        info!("🔍 Validating NAR engine functionality...");
        
        self.engine.validate()
            .context("NAR engine validation failed")?;
        
        let model_info = self.engine.get_model_info()
            .context("Failed to get model information")?;
        
        info!("📊 Model Information:");
        info!("   Context Size: {}", model_info["context_size"]);
        info!("   Embedding Size: {}", model_info["embedding_size"]);
        info!("   Vocabulary Size: {}", model_info["vocabulary_size"]);
        info!("   GPU Layers: {}", model_info["gpu_layers"]);
        
        Ok(())
    }

    /// Process a NAR service request
    pub async fn process_request(&mut self, request: NarServiceRequest) -> Result<NarServiceResponse> {
        let start_time = Instant::now();
        
        info!("🎯 Processing NAR request: {} (type: {:?})", request.id, request.request_type);
        
        // Convert to engine request
        let engine_request = GenerationRequest {
            prompt: self.format_prompt(&request),
            max_tokens: request.parameters.max_tokens,
            temperature: request.parameters.temperature,
            top_p: request.parameters.top_p,
            stop_sequences: request.parameters.stop_sequences.clone(),
        };

        // Generate response using the engine
        let engine_response = match self.engine.generate(engine_request) {
            Ok(response) => response,
            Err(e) => {
                error!("❌ Generation failed: {}", e);
                return Ok(NarServiceResponse {
                    id: Uuid::new_v4().to_string(),
                    request_id: request.id,
                    generated_text: String::new(),
                    tokens_used: 0,
                    processing_time_ms: start_time.elapsed().as_millis() as u64,
                    model_info: self.engine.get_model_info().unwrap_or_default(),
                    timestamp: Utc::now(),
                    status: ResponseStatus::Error(e.to_string()),
                });
            }
        };

        // Update statistics
        self.request_count += 1;
        self.total_tokens_generated += engine_response.tokens_used as u64;
        self.total_processing_time += Duration::from_millis(engine_response.processing_time_ms);

        let response = NarServiceResponse {
            id: Uuid::new_v4().to_string(),
            request_id: request.id,
            generated_text: engine_response.generated_text,
            tokens_used: engine_response.tokens_used,
            processing_time_ms: engine_response.processing_time_ms,
            model_info: self.engine.get_model_info().unwrap_or_default(),
            timestamp: Utc::now(),
            status: ResponseStatus::Success,
        };

        info!("✅ Request processed: {} tokens in {}ms", 
              response.tokens_used, response.processing_time_ms);

        Ok(response)
    }

    /// Format prompt based on request type
    fn format_prompt(&self, request: &NarServiceRequest) -> String {
        match request.request_type {
            NarRequestType::TextGeneration => {
                format!("Generate text: {}", request.prompt)
            }
            NarRequestType::QuestionAnswering => {
                format!("Question: {}\nAnswer:", request.prompt)
            }
            NarRequestType::CodeGeneration => {
                format!("Generate code for: {}\n```", request.prompt)
            }
            NarRequestType::GameNarrative => {
                format!("Bunkerverse Game Narrative: {}\n\nNarrative:", request.prompt)
            }
            NarRequestType::ChatCompletion => {
                format!("User: {}\nAssistant:", request.prompt)
            }
        }
    }

    /// Get service statistics
    pub fn get_statistics(&self) -> NarServiceStats {
        NarServiceStats {
            total_requests: self.request_count,
            total_tokens_generated: self.total_tokens_generated,
            total_processing_time: self.total_processing_time,
            average_tokens_per_request: if self.request_count > 0 {
                (self.total_tokens_generated as f64 / self.request_count as f64) as u64
            } else { 0 },
            average_processing_time_ms: if self.request_count > 0 {
                (self.total_processing_time.as_millis() / self.request_count as u128) as u64
            } else { 0 },
            model_path: self.model_path.clone(),
            config: self.config.clone(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NarServiceStats {
    pub total_requests: u64,
    pub total_tokens_generated: u64,
    pub total_processing_time: Duration,
    pub average_tokens_per_request: u64,
    pub average_processing_time_ms: u64,
    pub model_path: String,
    pub config: NarConfig,
}

// Performance testing
pub async fn run_performance_tests(service: &mut NarService) -> Result<PerformanceReport> {
    info!("🚀 Starting NAR service performance tests...");
    let mut report = PerformanceReport::default();
    
    // Test 1: Text Generation
    let text_gen_start = Instant::now();
    let text_requests = create_text_generation_requests(10)?;
    let mut text_gen_responses = Vec::new();
    
    for request in text_requests {
        let response = service.process_request(request).await?;
        text_gen_responses.push(response);
    }
    
    report.text_generation_time = text_gen_start.elapsed();
    report.text_generation_requests = text_gen_responses.len();
    report.text_generation_tokens = text_gen_responses.iter()
        .map(|r| r.tokens_used).sum();
    
    // Test 2: Question Answering
    let qa_start = Instant::now();
    let qa_requests = create_question_answering_requests(5)?;
    let mut qa_responses = Vec::new();
    
    for request in qa_requests {
        let response = service.process_request(request).await?;
        qa_responses.push(response);
    }
    
    report.qa_time = qa_start.elapsed();
    report.qa_requests = qa_responses.len();
    report.qa_tokens = qa_responses.iter()
        .map(|r| r.tokens_used).sum();
    
    // Test 3: Game Narrative Generation
    let narrative_start = Instant::now();
    let narrative_requests = create_narrative_requests(3)?;
    let mut narrative_responses = Vec::new();
    
    for request in narrative_requests {
        let response = service.process_request(request).await?;
        narrative_responses.push(response);
    }
    
    report.narrative_time = narrative_start.elapsed();
    report.narrative_requests = narrative_responses.len();
    report.narrative_tokens = narrative_responses.iter()
        .map(|r| r.tokens_used).sum();
    
    // Get final service statistics
    report.final_stats = service.get_statistics();
    
    info!("✅ NAR performance tests completed");
    Ok(report)
}

#[derive(Debug, Default)]
pub struct PerformanceReport {
    pub text_generation_time: Duration,
    pub text_generation_requests: usize,
    pub text_generation_tokens: usize,
    pub qa_time: Duration,
    pub qa_requests: usize,
    pub qa_tokens: usize,
    pub narrative_time: Duration,
    pub narrative_requests: usize,
    pub narrative_tokens: usize,
    pub final_stats: NarServiceStats,
}

impl PerformanceReport {
    pub fn print_summary(&self) {
        println!("\n🤖 NAR/GEMMA3-1B PoC - PERFORMANCE REPORT");
        println!("==========================================");
        
        println!("\n📝 Text Generation:");
        println!("  • Processed {} requests in {:?}", 
                 self.text_generation_requests, self.text_generation_time);
        println!("  • Generated {} tokens total", self.text_generation_tokens);
        if self.text_generation_time.as_secs_f64() > 0.0 {
            println!("  • Rate: {:.2} tokens/sec", 
                     self.text_generation_tokens as f64 / self.text_generation_time.as_secs_f64());
        }
        
        println!("\n❓ Question Answering:");
        println!("  • Processed {} requests in {:?}", 
                 self.qa_requests, self.qa_time);
        println!("  • Generated {} tokens total", self.qa_tokens);
        if self.qa_time.as_secs_f64() > 0.0 {
            println!("  • Rate: {:.2} tokens/sec", 
                     self.qa_tokens as f64 / self.qa_time.as_secs_f64());
        }
        
        println!("\n🎮 Game Narrative:");
        println!("  • Processed {} requests in {:?}", 
                 self.narrative_requests, self.narrative_time);
        println!("  • Generated {} tokens total", self.narrative_tokens);
        if self.narrative_time.as_secs_f64() > 0.0 {
            println!("  • Rate: {:.2} tokens/sec", 
                     self.narrative_tokens as f64 / self.narrative_time.as_secs_f64());
        }
        
        println!("\n📊 Overall Statistics:");
        println!("  • Total requests: {}", self.final_stats.total_requests);
        println!("  • Total tokens generated: {}", self.final_stats.total_tokens_generated);
        println!("  • Average tokens per request: {}", self.final_stats.average_tokens_per_request);
        println!("  • Average processing time: {}ms", self.final_stats.average_processing_time_ms);
        
        println!("\n🔧 Model Configuration:");
        println!("  • Model: {}", self.final_stats.model_path);
        println!("  • Context size: {}", self.final_stats.config.context_size);
        println!("  • Threads: {}", self.final_stats.config.threads);
        println!("  • GPU layers: {}", self.final_stats.config.gpu_layers);
        
        println!("==========================================");
    }
}

// Test data creation functions
fn create_text_generation_requests(count: usize) -> Result<Vec<NarServiceRequest>> {
    let prompts = vec![
        "The future of blockchain gaming",
        "Describe a virtual world",
        "Write about digital assets",
        "Explain decentralized systems",
        "The metaverse experience",
        "Virtual reality adventures", 
        "Digital collectibles story",
        "Blockchain technology benefits",
        "Gaming in Web3",
        "NFT marketplace features",
    ];
    
    let mut requests = Vec::new();
    for i in 0..count.min(prompts.len()) {
        requests.push(NarServiceRequest {
            id: Uuid::new_v4().to_string(),
            user_id: format!("user_{}", i),
            request_type: NarRequestType::TextGeneration,
            prompt: prompts[i].to_string(),
            parameters: NarParameters {
                max_tokens: 100,
                temperature: 0.7,
                ..Default::default()
            },
            timestamp: Utc::now(),
        });
    }
    
    Ok(requests)
}

fn create_question_answering_requests(count: usize) -> Result<Vec<NarServiceRequest>> {
    let questions = vec![
        "What is blockchain?",
        "How do NFTs work?",
        "What is a metaverse?",
        "Explain smart contracts",
        "What are digital assets?",
    ];
    
    let mut requests = Vec::new();
    for i in 0..count.min(questions.len()) {
        requests.push(NarServiceRequest {
            id: Uuid::new_v4().to_string(),
            user_id: format!("user_{}", i),
            request_type: NarRequestType::QuestionAnswering,
            prompt: questions[i].to_string(),
            parameters: NarParameters {
                max_tokens: 150,
                temperature: 0.5,
                ..Default::default()
            },
            timestamp: Utc::now(),
        });
    }
    
    Ok(requests)
}

fn create_narrative_requests(count: usize) -> Result<Vec<NarServiceRequest>> {
    let scenarios = vec![
        "Player enters the Bunkerverse for the first time",
        "A rare NFT is discovered in the digital wasteland",
        "Two players meet at the virtual marketplace",
    ];
    
    let mut requests = Vec::new();
    for i in 0..count.min(scenarios.len()) {
        requests.push(NarServiceRequest {
            id: Uuid::new_v4().to_string(),
            user_id: format!("user_{}", i),
            request_type: NarRequestType::GameNarrative,
            prompt: scenarios[i].to_string(),
            parameters: NarParameters {
                max_tokens: 200,
                temperature: 0.8,
                ..Default::default()
            },
            timestamp: Utc::now(),
        });
    }
    
    Ok(requests)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting NAR/Gemma3-1B PoC");
    
    // Initialize NAR service
    let mut service = NarService::new().await
        .context("Failed to initialize NAR service")?;
    
    // Run comprehensive performance tests
    let report = run_performance_tests(&mut service).await
        .context("Performance tests failed")?;
    
    // Print results
    report.print_summary();
    
    info!("🎉 NAR/Gemma3-1B PoC completed successfully!");
    
    Ok(())
}