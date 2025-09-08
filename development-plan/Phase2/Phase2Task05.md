Task 2.5: Netchain AI Runtime (NAR) - Functional Basic Contextual Narrative
(Live Full Schema P2 Default Context from Indexer, Secure FFI & Basic Resource Limits - Principles G, K, Q, R, P)
Technical Reference
Finalized ai_data_service.proto and indexer_service.proto API Contracts (v0.1)
Rust NAR Library (from Task 1.7) and its secure C-FFI (nar_ffi.h)
Game Design Documents (GDDs) for P2 MVE events and item details.
Context/Problem Statement
The NAR library from Phase 1 is functional but lacks context; it can only generate generic text. To fulfill its role as a core feature of the immersive experience, the NAR must be fed with real, contextual data about the events happening in the universe. This task enhances the NAR ecosystem by implementing the client-side logic to fetch live, on-chain state from the now-functional Indexer and use that data to engineer rich, dynamic prompts for the NAR to generate truly contextual narratives for core Phase 2 events.
Measurable Objectives
The client's Rust application logic is enhanced to fetch live context from the backend before calling the NAR library.
A set of P2-specific prompt templates is created that can be dynamically populated with this live data.
The NAR successfully generates narratives that incorporate specific details from live L3 events (e.g., a newly minted item's name and rarity).
A basic, functional resource limiting (timeout) mechanism is implemented for NAR inference calls to prevent client hangs.
Implementation Guidance
Action: Enhance the Rust NAR ecosystem (the Control Center Client's Rust application logic and the Rust NAR library itself) to enable functional, contextual text generation using llama.cpp. This will be triggered by core Phase 2 MVE events (e.g., NFT minting, robot linking). The key enhancement is that prompts will now be engineered using live P2 context (which contains full schema default data) fetched from the functional Rust Indexer via the functional Rust AI Data Service. Secure FFI handling and basic resource limits remain critical.
Context Acquisition & Prompt Engineering (in Control Center Client's Rust Application Logic - /client/rust-app-logic/src/nar/nar_prompt_builder.rs):
o This logic is triggered within the rustBackend QObject after a core P2 MVE action is confirmed (e.g., after a transaction is accepted by the L3 sequencer and the UI is ready for a narrative update).
o Context Acquisition:
The rustBackend calls its own internal method to fetch the latest AI context: self.ai_data_service_client.get_agent_context_for_nar(...). This calls the functional Rust AI Data Service (P1.6), which in turn calls the now-functional Rust Indexer (P2.4) to get the live P2 player state.
The context received will be an AIAgentInputDataContextProto containing the player's current live state, which for P2 includes: current_class, current_affiliation, final_stats, and details of any just-minted items.
o Dynamic Prompt Engineering for NAR (P2 Scope):
Create a set of P2-specific prompt templates as Rust string literals. These templates will use placeholders for the live context data.
Example Templates: (As defined in original document for NftMinted and RobotLinked events).
Implement a Rust function fn build_prompt(...) that selects the correct template and populates it with the live data from the context.
o Input Sanitization (Basic but Functional for P2 - Principle R): Before passing the final prompt string to the Rust NAR FFI, the Rust prompt builder will perform basic sanitization:
Truncate any free-text string fields from the context (like item names) to a safe length.
Strip common control characters that might interfere with llama.cpp's prompt processing.
Rust NAR FFI & Core Logic (/libs/nar-rust-wrapper-for-llama-cpp/src/lib.rs - Secure FFI & Basic Resource Limits):
o The secure C-FFI generate_text_nar_ffi (implemented in P1.7) receives the rich, sanitized prompt and generation parameters.
o The internal Rust nar_generate_text_internal function:
Validates lengths and basic content of the FFI-passed data.
Passes the rich prompt to the functional llama.cpp inference engine.
o Resource Limiting (Basic for P2): Implement a simple but functional timeout mechanism within the Rust wrapper for the llama.cpp inference call.
If the FFI call is async, use tokio::time::timeout.
If the FFI is sync, spawn the call on a separate thread and use thread::join with a timeout.
If the timeout (e.g., 15 seconds) is reached, the function must abort the operation and return a NAR_RESULT_TIMEOUT error code via the FFI.
Log a "TODO: Implement more granular CPU/memory limits for NAR in Phase 6."
o Returns the actual llama.cpp-generated text (or an error code) via the secure C-FFI with correct memory handling.
o Continue to ensure all FFI functions use std::panic::catch_unwind.
Logging & Testing:
o Unit Tests (Client's Rust App Logic):
Test the build_prompt function with various mock AIAgentInputDataContextProto structures. Verify the prompts are constructed correctly with live P2 default context.
Test the basic prompt sanitization logic.
o Unit Tests (Rust NAR Library):
Test the FFI layer with sanitized inputs that should trigger the basic timeout (by making the mocked FFI call sleep).
Test error code propagation for timeouts.
o Integration / Qualitative Evaluation (Client Rust App Logic calling Rust NAR lib with live data):
In an integration test setup, use the Admin CLI to submit a transaction to the L3 Smart Contracts (e.g., to mint a "Standard Explorer's Compass" Perk for a test player).
This updates the live Indexer Backend.
The test then invokes the client's Rust App Logic's function to generate a narrative for this event.
This function fetches the live context from the functional Indexer (via AI Data Service), builds the prompt, and calls the real Rust NAR library FFI.
Verification: Verify llama.cpp generates a narrative that correctly incorporates the specific P2 default details from the live Indexer. Verify the basic timeout prevents hangs if the inference call is artificially delayed.
Update docs/progress_logs/progress_phase_2.md:
Log the new prompt templates for P2 events.
Document the implementation of the Rust prompt builder, client-side sanitization, and the basic resource timeout in the NAR wrapper.
Provide examples of generated NAR outputs that reflect the specific P2 default details fetched from the live Indexer.
Reiterate the FFI security measures in place.
Document adherence to First Principles G, K, Q, R, P.
Design Rationale
This architecture makes the NAR highly adaptive. By separating the context acquisition (client Rust logic), prompt engineering (client Rust logic), and raw text generation (NAR library), we can easily add new narrative types in the future simply by adding new prompt templates and context queries, without having to change the core inference library. Basic resource limiting (timeout) is a critical resilience pattern to prevent a slow or hung AI model from freezing the entire client application.
Operational Considerations
Local-First: The entire flow will be tested in the local Docker Compose simulation. The performance of the NAR on the developer's machine will be the first baseline metric.
Cloud-Ready: The client-side nature of the NAR means its operational considerations are focused on the client build and distribution (handled in later phases), not the backend infrastructure.
Verification & Validation Criteria
Qualitative review shows generated narratives correctly and meaningfully use the live P2 default item/class/nexus specifics fetched from the Indexer.
The FFI is demonstrated to be robust in handling basic errors and the new timeout mechanism prevents client hangs during integration tests.
Testing Methodologies
A combination of unit tests for prompt logic and sanitization, and integration tests that run the full data pipeline (L3 event -> Indexer -> AI Data Service -> Client Logic -> NAR) to perform a final qualitative review of the generated text.
Version Control Strategy
Branching: The NAR enhancements are developed on a feature/ branch (e.g., feature/nar-contextual-p2).
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must review the prompt sanitization logic to prevent any form of prompt injection, especially from dynamic data fields like item names.
The implementation of the FFI timeout and catch_unwind are mandatory security and stability checkpoints.
ReviewedBy: AI Lead, Narrative Designer, Client Lead (Rust), Rust NAR Lead, Security Lead.
ReviewOutcome: Approved for P2 Core Event Narrative with Live P2 Default Context & Basic Secure FFI.
ValidationMethod: Qualitative review shows generated narratives are correct and meaningful. The FFI is demonstrated to be robust in handling basic errors and timeouts.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 2.5: Implemented Rust NAR Functional Basic Contextual Inference for Core P2 MVE Events (Live Full Schema P2 Default Context, Secure FFI with Basic Timeout)." @Phase2/