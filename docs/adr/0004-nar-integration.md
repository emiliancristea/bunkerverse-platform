# ADR-0004: NAR AI Integration Architecture

**Status:** Draft  
**Date:** 2024-09-08  
**Deciders:** Lead Architect, AI Lead  

## Context

The Bunkerverse Platform requires AI capabilities for natural language processing, content generation, and intelligent game assistance. We need a secure, high-performance integration with AI models while maintaining strict security boundaries.

## Decision

We will use **llama.cpp with Rust NAR FFI Wrapper** architecture:
- **llama.cpp**: High-performance C++ inference engine for LLM models
- **Rust NAR Wrapper**: Memory-safe FFI wrapper with security controls  
- **Gemma 3 1B Model**: Initial model choice for performance/capability balance
- **Security-First Design**: Comprehensive input validation and resource limiting

## Rationale

### Performance Requirements
- **Low Latency**: <100μs FFI call overhead requirement
- **Resource Efficiency**: Optimized memory usage for local deployment
- **Model Flexibility**: Support for multiple model formats and sizes
- **Real-time Processing**: Interactive response times for user queries

### Security Requirements
- **Memory Safety**: Rust wrapper prevents memory corruption attacks
- **Input Validation**: Comprehensive sanitization at FFI boundary
- **Resource Limiting**: Prevents DoS through resource exhaustion
- **Model Integrity**: SHA256 validation of model files

### Integration Benefits
- **Native Performance**: C++ inference engine optimized for performance
- **Memory Safety**: Rust wrapper eliminates memory corruption vulnerabilities
- **Type Safety**: Compile-time verification of FFI interfaces
- **Ecosystem**: Proven llama.cpp ecosystem with broad model support

## Consequences

### Positive
- High-performance AI inference with native C++ speed
- Memory-safe integration eliminates entire vulnerability classes
- Flexible model support enables future upgrades
- Local deployment ensures data privacy and low latency

### Negative
- Complex build system requiring both Rust and C++ toolchains
- FFI complexity increases debugging difficulty
- Model management overhead (downloads, validation, storage)
- Platform-specific compilation requirements

## Security Considerations & Threat Model Outline

### Threat Categories (STRIDE Analysis)

**Spoofing:**
- Model authenticity verification through SHA256 checksums
- Digital signatures for model distribution (future)
- Trusted model source validation

**Tampering:**
- Model file integrity validation before loading
- Runtime model state protection
- Secure model storage and access controls

**Repudiation:**
- Comprehensive logging of all AI operations and requests
- User session tracking for AI interactions
- Audit trail for model loading and configuration changes

**Information Disclosure:**
- Memory isolation between AI inference and application data
- Secure handling of user prompts and AI responses
- Prevention of model parameter extraction

**Denial of Service:**
- Resource limits on inference time and memory usage
- Request rate limiting per user and globally
- Timeout mechanisms for long-running inference

**Elevation of Privilege:**
- FFI boundary validation prevents privilege escalation
- Sandboxed AI operations with limited system access
- Secure handling of AI-generated content and commands

### Security Controls
- **FFI Security**: Comprehensive fuzz testing of FFI boundary (15+ test cases)
- **Input Validation**: Length limits, content filtering, encoding validation
- **Resource Limits**: Memory caps, timeout enforcement, CPU throttling
- **Model Security**: SHA256 validation, secure storage, access controls
- **Monitoring**: Real-time security monitoring and anomaly detection

### Fuzz Testing Results
- **Null Byte Injection**: ✅ BLOCKED
- **Buffer Overflow**: ✅ PREVENTED  
- **Malicious Patterns**: ✅ DETECTED
- **Oversized Inputs**: ✅ REJECTED
- **Memory Safety**: ✅ No crashes or leaks detected

## Dual-Mode (Off-Chain/On-Chain) Design Considerations

### MVE Mode (Off-Chain First)
- **Full AI Features**: All AI capabilities active for off-chain content
- **Local Processing**: AI inference runs locally for privacy
- **Content Generation**: AI assists with user content creation
- **Smart Recommendations**: AI-powered content and user matching

### Full Blockchain Mode
- **On-Chain AI**: AI-generated content can be minted as NFTs
- **Decentralized Models**: Support for decentralized AI model distribution
- **Token Integration**: AI usage potentially tokenized through NTC
- **Governance**: Community governance of AI model choices

### Configuration Management
```rust
pub struct NARConfig {
    pub model_path: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub enable_content_generation: bool,
    pub enable_blockchain_integration: bool,
}

// Security configuration
pub struct NARSecurityConfig {
    pub max_prompt_length: usize,
    pub timeout_seconds: u64,
    pub memory_limit_mb: usize,
    pub rate_limit_per_minute: u32,
}
```

## Architecture Details

### Component Architecture
```
┌─────────────────┐
│   User Input    │ ← Application Layer
├─────────────────┤
│ Input Validation│ ← Security Layer  
├─────────────────┤
│  Rust NAR FFI   │ ← Memory-Safe Bridge
├─────────────────┤
│   llama.cpp     │ ← High-Performance Inference
├─────────────────┤
│  Model (GGUF)   │ ← AI Model
└─────────────────┘
```

### Data Flow
1. **User Request**: Application receives AI request
2. **Validation**: Rust layer validates and sanitizes input  
3. **FFI Call**: Safe transition to C++ inference engine
4. **Inference**: llama.cpp processes request with model
5. **Response**: Results safely marshaled back to Rust
6. **Post-Processing**: Response validation and formatting

## Implementation Plan

### Phase 0 (Current)
- [x] PoC with Gemma 3 1B model integration
- [x] FFI security validation and fuzz testing
- [x] Performance benchmarking (<100μs overhead)
- [x] Model integrity validation (SHA256)

### Phase 1 (MVE)
- [ ] Production model deployment and management
- [ ] Advanced prompt engineering and fine-tuning
- [ ] User-facing AI features integration
- [ ] Performance monitoring and optimization

### Phase 2 (On-Chain Integration)
- [ ] AI-generated content tokenization
- [ ] Decentralized model distribution
- [ ] Community model governance
- [ ] Advanced AI marketplace features

## Model Selection Rationale

### Gemma 3 1B
- **Size**: 1B parameters balance capability with resource requirements
- **Performance**: Suitable for local inference on consumer hardware
- **License**: Permissive license suitable for commercial use
- **Community**: Strong community support and documentation

### Future Model Support
- **Larger Models**: Support for 7B+ models on high-end hardware
- **Specialized Models**: Task-specific models for different use cases
- **Custom Models**: Fine-tuned models for Bunkerverse-specific tasks
- **Multi-Modal**: Future support for vision and audio models

## Performance Benchmarks (PoC Results)
- **FFI Overhead**: <100μs (meets requirement)
- **Inference Speed**: ~50 tokens/second on target hardware
- **Memory Usage**: ~2GB peak with 1B parameter model
- **Model Loading**: ~3 seconds cold start
- **Security Validation**: All fuzz tests passed

## Alternatives Considered

### OpenAI API
- **Pros**: High-quality results, no local resource requirements
- **Cons**: Privacy concerns, API costs, internet dependency, rate limits

### Hugging Face Transformers (Python)
- **Pros**: Extensive model support, active ecosystem
- **Cons**: Python dependency, slower inference, FFI complexity

### Candle (Pure Rust)
- **Pros**: Memory safety, single language, modern architecture
- **Cons**: Less mature ecosystem, fewer optimizations, limited model support

### ONNX Runtime
- **Pros**: Cross-platform, good performance, broad model support
- **Cons**: Additional dependency, less specialized for LLMs

## Technical Requirements

### Build Dependencies
- **llama.cpp**: Latest stable (via git submodule)
- **Rust**: 1.80.0 with unsafe FFI capabilities
- **C++ Compiler**: MSVC 2022 or equivalent
- **CMake**: For llama.cpp compilation

### Runtime Requirements
- **Memory**: 4GB+ RAM recommended for 1B model
- **Storage**: 2GB+ for model files
- **CPU**: AVX2 support recommended for performance
- **GPU**: Optional CUDA support for acceleration

## References
- [llama.cpp Documentation](https://github.com/ggerganov/llama.cpp)
- [Gemma Model Documentation](https://ai.google.dev/gemma)
- [Security Assessment Report](../security/nar-security-assessment.md)
- [Performance Benchmarks](../performance/nar-benchmarks.md)

---

**Review Required By:** Security Lead, AI Lead, Performance Lead  
**Implementation Target:** Phase 0.2 (PoC Complete), Phase 1.0 (Production)