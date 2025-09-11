#ifndef BUNKERVERSE_NAR_LIBRARY_FFI_H
#define BUNKERVERSE_NAR_LIBRARY_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

// Version information
#define NAR_FFI_VERSION_MAJOR 0
#define NAR_FFI_VERSION_MINOR 1
#define NAR_FFI_VERSION_PATCH 0

// Maximum string lengths
#define NAR_MAX_MODEL_PATH_LEN 512
#define NAR_MAX_PROMPT_LEN 8192
#define NAR_MAX_RESPONSE_LEN 32768
#define NAR_MAX_ERROR_MESSAGE_LEN 256
#define NAR_MAX_CONTEXT_LEN 16384

// Result codes for NAR operations
typedef enum {
    NAR_SUCCESS = 0,
    NAR_ERROR_INVALID_PARAMS = 1,
    NAR_ERROR_MODEL_NOT_FOUND = 2,
    NAR_ERROR_MODEL_LOAD_FAILED = 3,
    NAR_ERROR_GENERATION_FAILED = 4,
    NAR_ERROR_OUT_OF_MEMORY = 5,
    NAR_ERROR_CONTEXT_TOO_LONG = 6,
    NAR_ERROR_ENGINE_NOT_INITIALIZED = 7,
    NAR_ERROR_ENGINE_ALREADY_INITIALIZED = 8,
    NAR_ERROR_THREAD_POOL_ERROR = 9,
    NAR_ERROR_TIMEOUT = 10,
    NAR_ERROR_CANCELLED = 11,
    NAR_ERROR_INVALID_UTF8 = 12,
    NAR_ERROR_UNKNOWN = 99
} NarResultCode;

// Engine status states
typedef enum {
    NAR_STATUS_UNINITIALIZED = 0,
    NAR_STATUS_INITIALIZING = 1,
    NAR_STATUS_READY = 2,
    NAR_STATUS_GENERATING = 3,
    NAR_STATUS_ERROR = 4,
    NAR_STATUS_SHUTTING_DOWN = 5
} NarEngineStatus;

// Configuration for NAR engine initialization
typedef struct {
    // Model configuration
    const char* model_path;                 // Path to Gemma3-1B model file
    uint32_t context_length;                // Maximum context length (tokens)
    uint32_t max_batch_size;                // Maximum batch size for generation
    bool use_gpu_acceleration;              // Enable GPU acceleration if available
    
    // Threading configuration
    uint32_t num_threads;                   // Number of CPU threads (0 = auto-detect)
    bool enable_thread_pool;                // Use thread pool for generation
    
    // Memory configuration
    uint64_t memory_limit_bytes;            // Maximum memory usage (0 = unlimited)
    bool enable_memory_mapping;             // Use memory mapping for model loading
    
    // Generation defaults
    float default_temperature;              // Default sampling temperature (0.0-2.0)
    uint32_t default_max_tokens;            // Default maximum tokens to generate
    float default_top_p;                    // Default nucleus sampling parameter
    uint32_t default_top_k;                 // Default top-k sampling parameter
    
    // Safety and validation
    bool enable_content_filtering;          // Enable content safety filtering
    bool validate_utf8;                     // Validate UTF-8 encoding
    uint32_t timeout_seconds;               // Generation timeout in seconds
    
    // Logging and debugging
    bool enable_debug_logging;              // Enable detailed logging
    const char* log_file_path;              // Log file path (NULL = stdout)
} NarConfigC;

// Parameters for text generation
typedef struct {
    // Input
    const char* prompt;                     // Input prompt text
    const char* context;                    // Optional conversation context
    
    // Generation parameters
    uint32_t max_tokens;                    // Maximum tokens to generate
    float temperature;                      // Sampling temperature (0.0-2.0)
    float top_p;                            // Nucleus sampling parameter (0.0-1.0)
    uint32_t top_k;                         // Top-k sampling parameter
    float repetition_penalty;               // Repetition penalty (1.0 = disabled)
    
    // Stopping conditions
    const char* stop_sequences[8];          // Up to 8 stop sequences (NULL-terminated)
    uint32_t stop_sequence_count;           // Number of stop sequences provided
    
    // Safety and filtering
    bool apply_content_filter;              // Apply content safety filter
    bool enable_profanity_filter;           // Filter profanity from output
    
    // Generation control
    uint32_t seed;                          // Random seed (0 = random)
    bool deterministic;                     // Ensure deterministic output
    uint32_t timeout_seconds;               // Generation timeout (0 = use default)
    
    // Advanced parameters
    float min_p;                            // Minimum probability threshold
    float typical_p;                        // Typical sampling parameter
    int32_t mirostat_mode;                  // Mirostat sampling mode (0=disabled, 1,2=enabled)
    float mirostat_tau;                     // Mirostat target entropy
    float mirostat_eta;                     // Mirostat learning rate
} GenerateParamsC;

// Generated text result
typedef struct {
    char* text;                             // Generated text (caller must free)
    uint32_t token_count;                   // Number of tokens generated
    uint32_t prompt_token_count;            // Number of tokens in prompt
    float generation_time_seconds;          // Time taken for generation
    uint32_t stop_reason;                   // Why generation stopped (0=max_tokens, 1=stop_sequence, 2=eos)
    bool content_filtered;                  // Whether content was filtered
    const char* stop_sequence_matched;      // Which stop sequence was matched (if any)
} NarGeneratedTextC;

// Engine status information
typedef struct {
    NarEngineStatus status;                 // Current engine status
    const char* status_message;             // Human-readable status description
    uint64_t total_memory_usage_bytes;      // Current memory usage
    uint64_t model_memory_usage_bytes;      // Memory used by loaded model
    uint32_t active_generations;            // Number of active generation requests
    uint32_t queued_generations;            // Number of queued generation requests
    float average_generation_time_seconds;  // Average generation time
    uint64_t total_generations_completed;   // Total generations since init
    uint64_t total_tokens_generated;        // Total tokens generated since init
    int64_t last_generation_timestamp;      // Unix timestamp of last generation
    bool gpu_acceleration_active;           // Whether GPU acceleration is being used
    const char* model_name;                 // Name/version of loaded model
    const char* error_message;              // Last error message (if status == ERROR)
} NarStatusReportC;

// Callback function type for streaming generation
typedef void (*NarStreamingCallback)(
    const char* partial_text,               // Partial generated text
    uint32_t token_count,                   // Tokens generated so far
    bool is_complete,                       // Whether generation is complete
    void* user_data                         // User-provided callback data
);

// ============================================================================
// Core NAR Engine Functions
// ============================================================================

/**
 * Initialize the NAR engine with specified configuration.
 * 
 * @param config Configuration parameters for engine initialization
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: NOT thread-safe, call only once during application startup
 * Memory: Allocates internal engine state, freed by shutdown_nar_engine_ffi()
 */
NarResultCode init_nar_engine_ffi(const NarConfigC* config);

/**
 * Generate text using the NAR engine.
 * 
 * @param params Generation parameters and input
 * @param result Output structure containing generated text (caller must free result->text)
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: Thread-safe after engine initialization
 * Memory: Allocates result->text, caller must call free_nar_generated_text_ffi()
 */
NarResultCode generate_text_nar_ffi(
    const GenerateParamsC* params,
    NarGeneratedTextC* result
);

/**
 * Generate text with streaming callback for real-time output.
 * 
 * @param params Generation parameters and input
 * @param callback Function to call with partial results
 * @param user_data User data passed to callback
 * @param final_result Final complete result (caller must free result->text)
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: Thread-safe after engine initialization
 * Memory: Allocates final_result->text, caller must call free_nar_generated_text_ffi()
 */
NarResultCode generate_text_streaming_nar_ffi(
    const GenerateParamsC* params,
    NarStreamingCallback callback,
    void* user_data,
    NarGeneratedTextC* final_result
);

/**
 * Free memory allocated by generate_text_nar_ffi().
 * 
 * @param result Result structure to free (sets text pointer to NULL)
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: Thread-safe
 * Memory: Frees result->text and resets structure
 */
NarResultCode free_nar_generated_text_ffi(NarGeneratedTextC* result);

/**
 * Get current status of the NAR engine.
 * 
 * @param status Output structure containing engine status
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation, status points to internal static data
 */
NarResultCode get_nar_status_ffi(NarStatusReportC* status);

/**
 * Shutdown the NAR engine and free all resources.
 * 
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: NOT thread-safe, call only during application shutdown
 * Memory: Frees all engine resources, invalidates all status pointers
 */
NarResultCode shutdown_nar_engine_ffi(void);

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Get version information for the NAR library.
 * 
 * @param major Output for major version number
 * @param minor Output for minor version number  
 * @param patch Output for patch version number
 * @return NAR_SUCCESS on success
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation
 */
NarResultCode get_nar_version_ffi(uint32_t* major, uint32_t* minor, uint32_t* patch);

/**
 * Get human-readable description of result code.
 * 
 * @param code Result code to describe
 * @return Static string describing the error (never NULL)
 * 
 * Thread Safety: Thread-safe
 * Memory: Returns pointer to static string, no allocation
 */
const char* get_nar_error_description_ffi(NarResultCode code);

/**
 * Cancel any ongoing text generation (if possible).
 * 
 * @return NAR_SUCCESS if cancellation requested, error code otherwise
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation
 * Note: Cancellation is best-effort and may not be immediate
 */
NarResultCode cancel_generation_ffi(void);

/**
 * Check if the NAR engine supports GPU acceleration on this system.
 * 
 * @param supported Output boolean for GPU support availability
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation
 */
NarResultCode check_gpu_support_ffi(bool* supported);

/**
 * Validate model file before attempting to load it.
 * 
 * @param model_path Path to model file to validate
 * @param is_valid Output boolean for model validity
 * @param error_message Buffer for error description (NAR_MAX_ERROR_MESSAGE_LEN bytes)
 * @return NAR_SUCCESS on success, error code on failure
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation, uses provided buffer
 */
NarResultCode validate_model_file_ffi(
    const char* model_path,
    bool* is_valid,
    char* error_message
);

// ============================================================================
// Default Configuration Helpers
// ============================================================================

/**
 * Initialize NarConfigC with safe default values.
 * 
 * @param config Configuration structure to initialize
 * @return NAR_SUCCESS on success
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation, initializes provided structure
 */
NarResultCode init_default_config_ffi(NarConfigC* config);

/**
 * Initialize GenerateParamsC with safe default values.
 * 
 * @param params Parameters structure to initialize
 * @return NAR_SUCCESS on success
 * 
 * Thread Safety: Thread-safe
 * Memory: No allocation, initializes provided structure
 */
NarResultCode init_default_params_ffi(GenerateParamsC* params);

#ifdef __cplusplus
}
#endif

#endif // BUNKERVERSE_NAR_LIBRARY_FFI_H