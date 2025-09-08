Task 0.2: Technology Choices & Core Libraries Finalization (with In-Depth, Security-Focused PoC Validation)
Technical Reference
Arbitrum Orbit Documentation
Docker & Docker Compose Documentation
Official documentation for all evaluated libraries (Redb, Axum, CXX-Qt, llama.cpp, etc.)
docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.md (specifically the SDL Charter)
Context/Problem Statement
Our architecture, defined in Task 0.1, relies on a complex stack of interoperating technologies. Foundational choices, such as the specific L3 framework, database solutions, and UI bridges, carry immense architectural weight. Making these commitments based on documentation alone is a recipe for disaster. This task mandates a rigorous, empirical validation phase where we build small, focused Proof-of-Concept (PoC) implementations for every critical and unproven technology. This process is designed to de-risk the project by replacing assumptions with hard data on performance, security, and integration feasibility before we commit to a technology for the entire MVE.
Measurable Objectives
A final, binding decision on using Arbitrum Orbit as the L3 framework, settling to Arbitrum One, is made and justified in a new ADR.
A signed-off PoC report, including a detailed security assessment, threat model, and performance benchmarks, is produced for every critical technology listed.
The DEPENDENCIES.md and COMPATIBILITY_MATRIX.md documents are populated with the final, validated library choices and their exact pinned versions.
Implementation Guidance
Action: Build focused, in-depth, time-boxed Proof-of-Concept (PoC) implementations for all critical technologies. These PoCs must rigorously evaluate functional suitability, performance against MVE targets, ease of integration, security implications, and maturity. Each PoC must include dedicated efforts to identify, prototype mitigations for, and document potential vulnerabilities. Based on PoC results, finalize all technology and core library selections. Update all related documentation (ADRs, DEPENDENCIES.md, COMPATIBILITY_MATRIX.md).
PoC Implementations:
Arbitrum Orbit L3 Framework PoC (CRITICAL):
Objective: Validate Arbitrum Orbit as the definitive framework for the BUNKERVERSE Netchain as a custom L3 chain settling to a public L2.
Tasks:
Following the official Orbit documentation, set up a local L3 development node.
Configure this local L3 chain to be anchored to a local Arbitrum One development node, which in turn is anchored to a local Ethereum testnet node (e.g., Anvil/Hardhat). This validates the full L3->L2->L1 settlement path.
Crucially, validate and document the exact technical process for configuring the NTC token as the native gas currency for the L3 chain.
Deploy a simple ERC-721 smart contract on the L3 and perform mint/transfer operations to test the full toolchain.
Success Criteria: A functional local L3 Orbit chain is running, correctly settling its state to the local L2. NTC is successfully configured as the gas token on the L3. Smart contract deployment and interaction on the L3 are successful. A new ADR (0008-l3-framework-selection.md) is created, justifying the choice of an Arbitrum Orbit L3.
Rust Service Storage PoC (Decision: Redb):
(This section remains unchanged)
Rust Indexer PoC (Elasticsearch/Typesense Integration):
(This section remains unchanged)
Rust NAR (llama.cpp via Gemma3 1B) - FFI-C++ PoC:
(This section remains unchanged)
zkLogin with Rust Identity Service PoC:
(This section remains unchanged)
Control Center Client (Rust App Logic with CXX-Qt & QML UI) PoC:
(This section remains unchanged)
Marketplace Service (Rust/Axum with Redis) PoC:
(This section remains unchanged)
Game Server Scalability (UE5 & Agones) PoC (Post-MVE Focus Prep):
(This section remains unchanged)
IPFS Integration PoC (for NFT Metadata):
(This section remains unchanged)
0.2.a. Mandatory Security Assessment & Hardening within each PoC Report:
(This section remains unchanged)
0.2.b. Iterative Threat Modeling (Focused, for each PoC component - Principle R):
(This section remains unchanged)
PoC Showcases & Documentation Update:
Conduct internal showcases for each completed PoC.
Update ADRs with finalized technology choices, detailed security assessments, and threat models.
Update DEPENDENCIES.md and COMPATIBILITY_MATRIX.md with all chosen libraries and their pinned versions.
The COMPATIBILITY_MATRIX.md must be populated with the finalized toolchain versions:
| Library/Tool | Version | Notes |
| :--- | :--- | :--- |
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | Pinned in Cargo.toml files. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Update docs/progress_logs/progress_phase_0.md with a detailed report for each PoC.
Reproducible PoC Execution & Artifact Capture (New Requirement):
(This section remains unchanged)
Design Rationale
This PoC phase is the most important risk-reduction activity in the entire plan. It replaces assumptions with hard data, allowing us to build on a foundation of proven technology rather than hope. Choosing the L3 framework is the most critical decision, hence its dedicated PoC. Front-loading security assessments and fuzz testing at this stage is exponentially cheaper than finding flaws in production.
Operational Considerations
The performance benchmarks from these PoCs will serve as the initial baseline for our production monitoring dashboards. All PoCs are conducted within the local Docker Compose simulation, which validates our ability to run a complex, multi-service environment on a single machine, a key requirement of the development plan. The parallel development of IaC for game servers with Agones on a local Kubernetes cluster (minikube) ensures we are cloud-ready even while developing locally.
Verification & Validation Criteria
Each PoC is successfully demonstrated to relevant leads. A signed-off PoC Report document exists for each technology, including a detailed Security Assessment, Threat Model, and performance benchmarks. Formal approval of technology choices is recorded in the ADRs.
Testing Methodologies
Each PoC will have its own small suite of tests to validate its core functionality and success criteria. The NAR FFI PoC will be subjected to extensive fuzz testing.
Version Control Strategy
Each PoC will be developed on a dedicated feature/poc-[technology] branch. Upon completion and approval, the learnings and finalized libraries are merged into develop via documentation updates and configuration changes.
Security Audit & Compliance Checkpoints
The Security Lead must review and sign off on the security assessment and threat model for every PoC report. The results of the fuzz tests for the NAR FFI are a mandatory security deliverable.
Git Commit: "Phase 0.2: Technology PoCs & Security Assessments Completed; Finalized Tech Stack (Arbitrum Orbit, Redb, Axum, CXX-Qt, IPFS, Gemma3, ES/TS) & Dependencies; ADRs Updated with PoC Validations & Security Mitigations."