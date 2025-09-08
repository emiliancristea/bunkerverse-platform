Task 1.7: Rust Netchain AI Runtime (NAR - llama.cpp via FFI)
(Functional Model Loading & Basic Secure FFI Inference - Client-Side Library for Gemma3 1B - Principles G, Q, R, P)
Technical Reference
* llama.cpp Documentation and llama.h header file
* Rust FFI Omnibus (for memory/safety patterns)
* cbindgen and cmake Rust crate documentation
* Finalized nar_ffi.h C-FFI contract (from Task 0.3)
Context/Problem Statement
The BUNKERVERSE vision relies on a deeply integrated, client-side AI to generate a living narrative. This requires a robust, secure, and performant bridge between our core Rust application logic and the high-performance C++ llama.cpp inference engine. This task focuses on building that foundational library. It involves compiling llama.cpp as a dependency, wrapping its unsafe C API in safe Rust abstractions, and exposing a hardened, secure C-style Foreign Function Interface (C-FFI) for the Control Center client to consume.
Measurable Objectives
* A functional Rust library (nar-rust-wrapper-for-llama-cpp) is created that successfully compiles the llama.cpp submodule as a static library dependency.
* The library can successfully load the pinned Gemma3 1B GGUF model and perform basic, functional inference.
* A secure C-FFI, matching nar_ffi.h, is implemented and exposed.
* The FFI boundary is hardened and validated against common security vulnerabilities (null pointers, malformed data, panics).
* Mandatory SHA256 checksum validation for the AI model file is implemented and functional.
Implementation Guidance
Action: Implement the core Rust NAR library in /libs/nar-rust-wrapper-for-llama-cpp/src/lib.rs. This library will functionally wrap the llama.cpp C/C++ library, enabling the loading of the Gemma3 1B GGUF model and exposing a secure C-style Foreign Function Interface (C-FFI) that allows the Control Center Client's Rust application logic to perform basic, actual, but simple inference. This task focuses on creating a robust, secure, and functional bridge between the Rust ecosystem and the C++ llama.cpp engine.
* llama.cpp Integration (Rust build.rs & FFI Bindings - Functional & Secure):
o Finalize build.rs script: This script is crucial for seamless integration.
* It will use the cmake crate to configure and compile the llama.cpp Git submodule (pinned version from P0) as a static library (libllama.a or llama.lib).
* It will pass secure compilation flags to llama.cpp's CMake build, such as -DCMAKE_BUILD_TYPE=Release, -DLLAMA_STATIC=ON.
* The script will instruct Cargo to link the Rust NAR wrapper against the compiled libllama.a.
o Finalize bindgen or Manual FFI Bindings:
* Use the bindgen crate (configured in build.rs) to automatically generate safe Rust FFI bindings from llama.h.
* All FFI calls from Rust to these llama.cpp functions must be wrapped in unsafe blocks. Each unsafe block must be accompanied by a comment explaining the safety invariants being upheld.
* Rust NAR Core Logic (wrapping llama.cpp calls functionally & securely):
o Define Rust structs to manage llama.cpp resources, ensuring automatic cleanup via the Drop trait.
Generated rust
// Example struct to hold a llama_model pointer
pub struct LlamaModel {
    ptr: *mut llama_model,
}
impl Drop for LlamaModel {
    fn drop(&mut self) {
        // SAFETY: self.ptr is a valid model pointer.
        unsafe { llama_free_model(self.ptr); }
    }
}
// Similar struct for LlamaContext
Use code with caution.Rust
o Implement high-level Rust functions that encapsulate the llama.cpp FFI calls (e.g., nar_load_model, nar_new_context).
o Implement the core inference logic in fn nar_generate_text_internal(...):
* Input Validation (Rust wrapper): Validate prompt_str length and the basic structure of generation parameters received from the FFI layer.
* Tokenization: Call llama_tokenize via FFI.
* Evaluation Loop: Implement a basic inference loop calling llama_eval and llama_sample_token_greedy (or other samplers).
* Detokenization: Collect generated tokens and use llama_token_to_piece via FFI.
* Error Handling: Robustly handle potential errors from all llama.cpp FFI calls.
* C-FFI Implementation (Exposing Rust NAR to Client - nar_ffi.h from cbindgen - Secure):
o In /libs/nar-rust-wrapper-for-llama-cpp/src/ffi.rs, implement all extern "C" functions as defined in P0.3.
o FFI Security Measures (Principle R - Mandatory Implementation):
* Input Handling: All C strings (*const c_char) are immediately and safely converted to Rust types using CStr::from_ptr. Null pointer checks are mandatory. All C structs passed by pointer are validated upon receipt.
* Output Memory Management: Narrative text returned as *mut c_char is allocated by Rust using CString::new(rust_string).into_raw(). The consumer (the Client's Rust app logic) is given ownership and must call free_nar_generated_text_ffi to deallocate it. This ownership contract is documented clearly in nar_ffi.h.
* Panic Safety: Every single extern "C" FFI function must wrap its entire body in std::panic::catch_unwind. If a panic occurs, the function must catch the panic and return a specific error code (e.g., NAR_RESULT_PANIC) to the caller.
* Error Codes: All FFI functions will return a clear, comprehensive NarResultCode enum.
* Model File Handling (Manual Download & Config for Phase 1 - Principle Q stub):
* For P1, the developer will manually download the pinned Gemma3 1B GGUF model and place it in a known directory within the client project.
* The client's configuration will point to this local file path.
* The init_nar_engine_ffi function will receive this path in its NarConfigC struct.
* Inside the Rust NAR library, before loading the model, implement a function to calculate the SHA256 checksum of the file at the given path and compare it against a hardcoded, known-good checksum. If the checksums do not match, init_nar_engine_ffi must fail and return a NAR_RESULT_MODEL_VALIDATION_FAILED error code.
* Logging (tracing) & Testing:
* Use the tracing crate within the Rust NAR library to log key events.
* Unit Tests (Rust NAR Library): Test llama.cpp FFI wrappers with mocked FFI functions (mockall). Test error handling logic. Test the model checksum validation logic.
* FFI Integration Test (Client's Rust App Logic calling compiled Rust NAR lib): In the /client/rust-app-logic crate, create integration tests that link against the compiled Rust NAR library and call the C-FFI functions.
* Security Negative Tests for FFI: Pass null pointers, overly long strings, and malformed config structs to FFI functions to verify robust error code returns and no crashes. Verify the full memory management cycle (generate -> free) with tools like Valgrind in the local Docker Compose simulation.
* Functional Test: Perform a full functional test: init -> generate -> free -> shutdown. Verify a coherent narrative is generated.
* Update docs/progress_logs/progress_phase_1.md:
* Log the finalized build.rs script.
* Detail the Rust FFI bindings and the high-level Rust wrappers.
* Provide a detailed breakdown of the secure C-FFI implementation, including code snippets demonstrating the mandatory security measures.
* Document the pinned llama.cpp version (Git commit SHA) and the Gemma3 model version and its SHA256 checksum.
* Document adherence to First Principles G, Q, R, P.
Design Rationale
Wrapping the raw C++ library in safe Rust abstractions is a core tenet of the project's security philosophy. This contains the unsafe FFI calls to a small, well-audited module. A secure FFI with a clear memory management contract and robust panic safety is non-negotiable for client stability. Mandatory checksum validation of the AI model prevents a class of supply chain attacks where a malicious model could be distributed to users.
Operational Considerations
* Local-First: The llama.cpp library will be compiled from source as part of the cargo build process for the client. The GGUF model file will be treated as a local asset that must be present for the client to run.
* Cloud-Ready: The final client installer (built in Phase 6) will need to either bundle the GGUF model or use the robust nar-model-handler to download it on first run.
Verification & Validation Criteria
* All unit and integration tests, including FFI security negative tests, pass.
* A successful demonstration shows the Client's Rust app logic calling the Rust NAR C-FFI to load the Gemma3 1B model, perform basic secure inference, and correctly manage memory.
* The FFI boundary is validated to be robust against common C-interop errors. The security review for the FFI is signed off.
Testing Methodologies
A combination of unit tests with mocked FFI calls (mockall), integration tests linking the real library, security negative tests focusing on the FFI boundary, and functional tests to ensure coherent output.
Version Control Strategy
* Branching: The library is developed on a feature/nar-library branch.
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* A mandatory, detailed security peer review of the FFI implementation (ffi.rs) is required, focusing on input validation, memory management, and panic safety.
* The SHA256 model validation logic must be reviewed and approved.
* The Security Lead's sign-off on the overall FFI security posture is non-negotiable.
ReviewedBy: AI Lead, Lead Rust Developer, Client Lead (as consumer), Security Lead (mandatory reviewer for FFI security, llama.cpp interaction, and memory management).
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests pass. A successful demonstration shows the Client's Rust app logic calling the Rust NAR C-FFI. The FFI boundary is validated to be robust. Security review for FFI is signed off.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.7: Implemented Rust NAR Core (llama.cpp Wrapper & Secure C-FFI - Functional Gemma3 1B Model Loading & Basic Secure Inference)." @Phase1/

------------------------------------------------------------------------------------------------------------------
