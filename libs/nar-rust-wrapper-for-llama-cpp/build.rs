use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");

    // For PoC, we'll create a simple mock FFI interface
    // In a real implementation, this would build llama.cpp and generate bindings

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Create a mock header for bindgen
    let mock_header = r#"
typedef struct llama_context llama_context;
typedef struct llama_model llama_model;

struct llama_model_params {
    int n_gpu_layers;
    int main_gpu;
    float *tensor_split;
    int vocab_only;
    int use_mmap;
    int use_mlock;
};

struct llama_context_params {
    unsigned int seed;
    int n_ctx;
    int n_batch;
    int n_threads;
    int n_threads_batch;
    float rope_freq_base;
    float rope_freq_scale;
    int n_gqa;
    float rms_norm_eps;
    int rope_scaling_type;
    int yarn_ext_factor;
    int yarn_attn_factor;
    float yarn_beta_fast;
    float yarn_beta_slow;
    int yarn_orig_ctx;
    int embedding;
    int offload_kqv;
};

typedef int llama_token;

// Mock function declarations for PoC
llama_model* llama_load_model_from_file(const char* path_model, struct llama_model_params params);
void llama_free_model(llama_model* model);
llama_context* llama_new_context_with_model(llama_model* model, struct llama_context_params params);
void llama_free(llama_context* ctx);
int llama_tokenize(llama_context* ctx, const char* text, int text_len, llama_token* tokens, int n_max_tokens, int add_bos, int special);
int llama_decode(llama_context* ctx, llama_token* tokens, int n_tokens, int n_past, int n_threads);
llama_token llama_sample_token_greedy(llama_context* ctx, llama_token* candidates, int n_candidates);
const char* llama_token_to_piece(llama_context* ctx, llama_token token);
int llama_n_ctx(llama_context* ctx);
int llama_n_embd(llama_context* ctx);
int llama_n_vocab(llama_context* ctx);
"#;

    std::fs::write(out_path.join("llama_mock.h"), mock_header).unwrap();

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header(out_path.join("llama_mock.h").to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // For PoC, we'll create mock implementations
    let mock_impl = r#"
// Mock implementations for PoC testing
// In real implementation, these would link to actual llama.cpp

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

typedef int llama_token;

struct llama_model_params {
    int n_gpu_layers;
    int main_gpu;
    float *tensor_split;
    int vocab_only;
    int use_mmap;
    int use_mlock;
};

struct llama_context_params {
    unsigned int seed;
    int n_ctx;
    int n_batch;
    int n_threads;
    int n_threads_batch;
    float rope_freq_base;
    float rope_freq_scale;
    int n_gqa;
    float rms_norm_eps;
    int rope_scaling_type;
    int yarn_ext_factor;
    int yarn_attn_factor;
    float yarn_beta_fast;
    float yarn_beta_slow;
    int yarn_orig_ctx;
    int embedding;
    int offload_kqv;
};

typedef struct llama_context {
    int dummy;
} llama_context;

typedef struct llama_model {
    int dummy;
} llama_model;

llama_model* llama_load_model_from_file(const char* path_model, struct llama_model_params params) {
    printf("[MOCK] Loading model from: %s\n", path_model);
    llama_model* model = malloc(sizeof(llama_model));
    model->dummy = 42;
    return model;
}

void llama_free_model(llama_model* model) {
    printf("[MOCK] Freeing model\n");
    free(model);
}

llama_context* llama_new_context_with_model(llama_model* model, struct llama_context_params params) {
    printf("[MOCK] Creating context with n_ctx=%d\n", params.n_ctx);
    llama_context* ctx = malloc(sizeof(llama_context));
    ctx->dummy = 123;
    return ctx;
}

void llama_free(llama_context* ctx) {
    printf("[MOCK] Freeing context\n");
    free(ctx);
}

int llama_tokenize(llama_context* ctx, const char* text, int text_len, llama_token* tokens, int n_max_tokens, int add_bos, int special) {
    printf("[MOCK] Tokenizing: %.*s\n", text_len, text);
    // Mock tokenization - just create some dummy tokens
    int n_tokens = text_len < n_max_tokens ? text_len : n_max_tokens;
    for (int i = 0; i < n_tokens; i++) {
        tokens[i] = (llama_token)(text[i] + 100);
    }
    return n_tokens;
}

int llama_decode(llama_context* ctx, llama_token* tokens, int n_tokens, int n_past, int n_threads) {
    printf("[MOCK] Decoding %d tokens\n", n_tokens);
    return 0; // Success
}

llama_token llama_sample_token_greedy(llama_context* ctx, llama_token* candidates, int n_candidates) {
    printf("[MOCK] Sampling greedy token\n");
    return candidates[0]; // Just return first candidate
}

const char* llama_token_to_piece(llama_context* ctx, llama_token token) {
    static char buffer[256];
    snprintf(buffer, sizeof(buffer), "[TOKEN_%d]", token);
    return buffer;
}

int llama_n_ctx(llama_context* ctx) { return 2048; }
int llama_n_embd(llama_context* ctx) { return 4096; }
int llama_n_vocab(llama_context* ctx) { return 32000; }
"#;

    std::fs::write(out_path.join("llama_mock.c"), mock_impl).unwrap();

    // Compile the mock implementation
    cc::Build::new()
        .file(out_path.join("llama_mock.c"))
        .compile("llama_mock");

    println!("cargo:rustc-link-lib=llama_mock");
}
