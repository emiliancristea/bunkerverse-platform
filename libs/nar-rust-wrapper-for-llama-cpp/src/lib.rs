use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::ptr;

// Include the generated bindings
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;

/// Configuration for the NAR model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarConfig {
    pub model_path: String,
    pub context_size: u32,
    pub gpu_layers: i32,
    pub threads: i32,
    pub batch_size: i32,
    pub seed: u32,
}

impl Default for NarConfig {
    fn default() -> Self {
        Self {
            model_path: "models/gemma-3-1b.gguf".to_string(),
            context_size: 2048,
            gpu_layers: 0,
            threads: 4,
            batch_size: 512,
            seed: 42,
        }
    }
}

/// Request for text generation
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub stop_sequences: Vec<String>,
}

/// Response from text generation
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub generated_text: String,
    pub tokens_used: usize,
    pub processing_time_ms: u64,
}

/// NAR (Natural AI Reasoning) wrapper for llama.cpp
pub struct NarEngine {
    model: *mut llama_model,
    context: *mut llama_context,
    config: NarConfig,
}

impl NarEngine {
    /// Create a new NAR engine with the specified configuration
    pub fn new(config: NarConfig) -> Result<Self> {
        unsafe {
            // Initialize model parameters
            let model_params = llama_model_params {
                n_gpu_layers: config.gpu_layers,
                main_gpu: 0,
                tensor_split: ptr::null_mut(),
                vocab_only: 0,
                use_mmap: 1,
                use_mlock: 0,
            };

            // Load the model
            let model_path = CString::new(config.model_path.clone())
                .map_err(|e| anyhow!("Invalid model path: {}", e))?;

            let model = llama_load_model_from_file(model_path.as_ptr(), model_params);
            if model.is_null() {
                return Err(anyhow!("Failed to load model from: {}", config.model_path));
            }

            // Initialize context parameters
            let context_params = llama_context_params {
                seed: config.seed,
                n_ctx: config.context_size as i32,
                n_batch: config.batch_size,
                n_threads: config.threads,
                n_threads_batch: config.threads,
                rope_freq_base: 10000.0,
                rope_freq_scale: 1.0,
                n_gqa: 1,
                rms_norm_eps: 1e-5,
                rope_scaling_type: 0,
                yarn_ext_factor: -1,
                yarn_attn_factor: 1,
                yarn_beta_fast: 32.0,
                yarn_beta_slow: 1.0,
                yarn_orig_ctx: 0,
                embedding: 0,
                offload_kqv: 1,
            };

            // Create context
            let context = llama_new_context_with_model(model, context_params);
            if context.is_null() {
                llama_free_model(model);
                return Err(anyhow!("Failed to create llama context"));
            }

            Ok(NarEngine {
                model,
                context,
                config,
            })
        }
    }

    /// Generate text based on the given prompt
    pub fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse> {
        let start_time = std::time::Instant::now();

        unsafe {
            // Tokenize the prompt
            let prompt_cstr = CString::new(request.prompt.clone())
                .map_err(|e| anyhow!("Invalid prompt: {}", e))?;

            let mut tokens = vec![0 as llama_token; 1024];
            let n_tokens = llama_tokenize(
                self.context,
                prompt_cstr.as_ptr(),
                request.prompt.len() as i32,
                tokens.as_mut_ptr(),
                tokens.len() as i32,
                1, // add_bos
                0, // special
            );

            if n_tokens < 0 {
                return Err(anyhow!("Failed to tokenize prompt"));
            }

            tokens.truncate(n_tokens as usize);

            // Decode the prompt tokens
            let decode_result = llama_decode(
                self.context,
                tokens.as_mut_ptr(),
                tokens.len() as i32,
                0, // n_past
                self.config.threads,
            );

            if decode_result != 0 {
                return Err(anyhow!("Failed to decode prompt tokens"));
            }

            // Generate tokens one by one
            let mut generated_tokens = Vec::new();
            let mut generated_text = String::new();

            for _ in 0..request.max_tokens {
                // Sample next token (greedy for simplicity in PoC)
                let next_token = llama_sample_token_greedy(
                    self.context,
                    tokens.as_mut_ptr(),
                    tokens.len() as i32,
                );

                generated_tokens.push(next_token);

                // Convert token to text
                let token_str = llama_token_to_piece(self.context, next_token);
                let token_text = CStr::from_ptr(token_str).to_string_lossy();

                // Check for stop sequences
                let should_stop = request
                    .stop_sequences
                    .iter()
                    .any(|stop| token_text.contains(stop));

                if should_stop {
                    break;
                }

                generated_text.push_str(&token_text);

                // Prepare for next iteration
                tokens.push(next_token);
                if tokens.len() >= self.config.context_size as usize {
                    break;
                }

                // Decode the new token
                let decode_result = llama_decode(
                    self.context,
                    [next_token].as_mut_ptr(),
                    1,
                    tokens.len() as i32 - 1,
                    self.config.threads,
                );

                if decode_result != 0 {
                    break;
                }
            }

            let processing_time = start_time.elapsed().as_millis() as u64;

            Ok(GenerationResponse {
                generated_text,
                tokens_used: generated_tokens.len(),
                processing_time_ms: processing_time,
            })
        }
    }

    /// Get model information
    pub fn get_model_info(&self) -> Result<serde_json::Value> {
        unsafe {
            Ok(serde_json::json!({
                "context_size": llama_n_ctx(self.context),
                "embedding_size": llama_n_embd(self.context),
                "vocabulary_size": llama_n_vocab(self.context),
                "model_path": self.config.model_path,
                "gpu_layers": self.config.gpu_layers,
                "threads": self.config.threads
            }))
        }
    }

    /// Validate model loading and basic functionality
    pub fn validate(&self) -> Result<()> {
        let test_request = GenerationRequest {
            prompt: "Hello".to_string(),
            max_tokens: 5,
            temperature: 0.7,
            top_p: 0.9,
            stop_sequences: vec![],
        };

        let response = self.generate(test_request)?;

        if response.tokens_used == 0 {
            return Err(anyhow!("Model validation failed: no tokens generated"));
        }

        println!("✅ NAR Engine validation successful");
        println!("   Tokens generated: {}", response.tokens_used);
        println!("   Processing time: {}ms", response.processing_time_ms);
        println!("   Sample output: {}", response.generated_text);

        Ok(())
    }
}

impl Drop for NarEngine {
    fn drop(&mut self) {
        unsafe {
            if !self.context.is_null() {
                llama_free(self.context);
            }
            if !self.model.is_null() {
                llama_free_model(self.model);
            }
        }
    }
}

// Thread safety markers (in real implementation, need proper synchronization)
unsafe impl Send for NarEngine {}
unsafe impl Sync for NarEngine {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nar_config_default() {
        let config = NarConfig::default();
        assert_eq!(config.context_size, 2048);
        assert_eq!(config.threads, 4);
    }

    #[test]
    fn test_generation_request_serialization() {
        let request = GenerationRequest {
            prompt: "Test prompt".to_string(),
            max_tokens: 100,
            temperature: 0.7,
            top_p: 0.9,
            stop_sequences: vec!["<stop>".to_string()],
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GenerationRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.prompt, deserialized.prompt);
        assert_eq!(request.max_tokens, deserialized.max_tokens);
    }

    #[test]
    fn test_mock_nar_engine() {
        // This test uses mock implementation
        let config = NarConfig {
            model_path: "mock://model".to_string(),
            ..Default::default()
        };

        // In PoC, we can't actually test the engine creation due to mock limitations
        // But we can test the configuration and serialization
        let json = serde_json::to_string(&config).unwrap();
        let parsed: NarConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.model_path, parsed.model_path);
    }
}
