Task 1.11: C++ Control Center Client - Core Communication Bridges Finalization
(For P1 Scope - Secure zkLogin, Initial Data, Functional Secure NAR FFI Call - Principles P, R)
Technical Reference
* CXX-Qt Documentation
* Rust FFI Omnibus (for panic safety and error handling patterns)
* Finalized nar_ffi.h C-FFI contract (from Task 0.7)
* docs/developer_guides/SECURE_FFI_PATTERNS.md
Context/Problem Statement
The Control Center Client's architecture is a complex tripartite system consisting of the C++ Qt Shell, the core Rust Application Logic, and the QML UI. While the individual components have been implemented, the communication bridges between them (CXX-Qt for Rust?QML, C-FFI for Rust?C++) are the highest-risk areas for bugs, crashes, and security vulnerabilities. This task is a dedicated finalization and hardening pass to ensure these bridges are robust, secure, and function correctly for all Phase 1 features.
Measurable Objectives
* All P1-scope FFI/bridge methods and properties are formally reviewed and hardened.
* The end-to-end error propagation chain (e.g., from NAR FFI to QML UI) is finalized and validated.
* Strict input validation is re-verified and documented for all data passed from QML to Rust.
* All unit and integration tests for the communication bridges pass, demonstrating robust and secure data/signal flow.
Implementation Guidance
Action: Finalize and harden the core communication bridges within the Control Center Client. This involves ensuring the C++ Qt Shell, the Rust Application Logic (via CXX-Qt), and the QML UI can communicate robustly and securely for all Phase 1 features: orchestrating the zkLogin flow, fetching initial player data, and making a basic functional call to the Rust NAR library's FFI.
* Finalizing the Communication Bridges:
o QML <-> Rust Application Logic (via CXX-Qt): This is the primary bridge for UI logic and application state.
* Final Review: Conduct a final review of all #[cxx_qt::qinvokable] Rust methods and #[cxx_qt::qproperty] Rust properties exposed to QML for the P1 scope.
* P1 Scope Functions/Properties to Finalize:
* rustBackend.getLoginProviders()
* rustBackend.initiateZkLogin(provider_id)
* rustBackend.processOauthCallbackUrl(url)
* rustBackend.fetchInitialPlayerData()
* rustBackend.generateLocalNarrative(promptJson)
* Properties like rustBackend.playerProfile and rustBackend.aiContext.
* Signals like loginSucceeded, loginFailed, dataFetchSucceeded, dataFetchFailed, narNarrativeReady.
* Security Hardening: At this integration finalization step, re-verify that the Rust implementation of each invokable method performs strict input validation on all data passed from QML. For example, generateLocalNarrative(promptJson) must validate that promptJson is well-formed JSON and does not contain malicious content before passing it to the NAR library.
o Rust Application Logic <-> C++ Shell (via CXX-Qt): This bridge is for actions the Rust logic cannot perform directly, which for P1 is primarily opening a URL.
* Finalize the mechanism for the Rust initiate_zk_login method to signal the C++ Shell to open the OAuth URL. A CXX-Qt signal emitted from Rust that a C++ slot connects to is a clean approach.
o Rust Application Logic <-> Rust NAR Library (via C-FFI):
* Finalize the Rust code within the rustBackend that consumes the NAR library's C-FFI (as detailed in P1.7).
* #[cxx_qt::qinvokable] async fn generate_local_narrative(&self, prompt_json: &str):
* This new Rust method is exposed to QML.
* It parses the prompt_json string.
* It calls the Rust NAR library's C-FFI functions (generate_text_nar_ffi).
* It handles the returned string and error codes securely (including freeing memory).
* It emits a narNarrativeReady(narrative_string) or narNarrativeFailed(error_message) signal to QML.
* Error Handling Propagation (End-to-End):
o Finalize the end-to-end error propagation chain. For example:
* Rust NAR FFI returns an error code.
* The Rust App Logic's NAR consumer catches this, converts it to a NarError enum.
* The generate_local_narrative method returns this error.
* The CXX-Qt bridge logic converts this Rust error into a QML narNarrativeFailed signal with an error message payload.
* The QML UI listens for this signal and displays the error.
o Ensure this pattern is robust for all P1 interactions (zkLogin, data fetching, NAR).
* Testing:
o Unit Tests (Rust App Logic & C++ Shell):
* Test the input validation logic within the rustBackend's invokable methods (e.g., test generate_local_narrative with malformed JSON).
* Test the error propagation logic, ensuring Rust errors are correctly converted to QML signals.
o Integration Tests (Simulating Full C++/Rust/QML Interaction):
* Use Qt Test to write integration tests that load the QML UI, which in turn calls the real rustBackend object.
* Mock the external dependencies of rustBackend (gRPC services, NAR FFI).
* Scenario 1 (Login): Simulate a button click in QML, verify the initiate_zk_login Rust method is called, and that the signal to the C++ Shell to open a URL is correctly emitted.
* Scenario 2 (NAR Call): Simulate a debug button click in QML that calls rustBackend.generate_local_narrative. Verify the Rust logic correctly calls the mocked NAR FFI, handles the response, and emits the narNarrativeReady signal, which the QML test then verifies.
* Update docs/progress_logs/progress_phase_1.md:
o Log the finalized communication interfaces between C++, Rust, and QML for the P1 scope.
o Document the input validation and sanitization steps added at the CXX-Qt bridge layer (Rust side) for data coming from QML.
o Detail the end-to-end error propagation mechanism.
Design Rationale
A dedicated finalization and hardening task for the client's communication bridges is a critical security and stability measure. These FFI boundaries are the most complex and potentially fragile parts of the client architecture. Ensuring they are robust, panic-safe, and handle errors gracefully is essential for a high-quality user experience and a secure application. This task transforms the individually developed components into a single, cohesive, and resilient whole.
Operational Considerations
The patterns for error propagation and FFI security established in this task will become the standard for all future client development. Any new functions exposed to QML or new FFI interactions must adhere to these hardened patterns.
Verification & Validation Criteria
* All unit and integration tests for the C++/Rust/QML communication bridges pass.
* A successful live demonstration shows robust and secure data and signal flow between all three layers for the defined P1 features (zkLogin, data fetch, NAR call), including graceful handling of mocked errors.
Testing Methodologies
* Unit Tests: Focus on testing the "glue" code in the Rust rustBackend, mocking all external dependencies to test validation and error conversion logic in isolation.
* Integration Tests: Use the Qt Test framework to simulate real QML interactions with the actual Rust rustBackend object, allowing for validation of the full QML?Rust bridge.
Version Control Strategy
* Branching: This finalization work will be done on the respective feature branches (e.g., feature/client-auth-flow) before they are merged.
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* A mandatory security review of all finalized FFI/bridge code is required before this task is complete.
* The Security Lead must sign off on the end-to-end error propagation logic, ensuring it does not leak sensitive information.
* The input validation at the QML-to-Rust boundary is a critical security checkpoint.
ReviewedBy: Client Lead (C++ & Rust & QML), Rust NAR Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests for the C++/Rust/QML communication bridges pass. A successful demonstration shows robust and secure data and signal flow between all three layers.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.11: Finalized CC Client Core Communication Bridges (C++/Rust/QML via CXX-Qt) for Phase 1 Scope (incl. Secure NAR FFI Call Path)." @Phase1/

------------------------------------------------------------------------------------------------------------------
