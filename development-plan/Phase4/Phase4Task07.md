Task 4.7: Rust Netchain AI Runtime (NAR) - Full Secure Contextual Narrative
(All Item/Class/Stat/Nexus Details from Live Indexer Data - Principles G, K, Q, R, P)
Technical Reference
Finalized ai_data_service.proto API Contract (enhanced in Task 4.6)
Rust NAR Library and secure C-FFI (from Task 1.7)
Game Design Documents (GDDs) for narrative tone and class/item specifics

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
Context/Problem Statement
The NAR's narratives in previous phases were basic, reacting to simple events with default data. Now that the Indexer (Task 4.6) serves the full, live, dynamic state of a player's Bunkerguard (class, affiliation, stats, equipped items), the NAR ecosystem must be significantly enhanced to leverage this rich data. This task focuses on implementing the advanced context acquisition and prompt engineering required to generate deeply contextual, specific, and high-quality narratives for all Guard and Marketplace events, making the AI feel truly aware of the player's identity and actions.
Measurable Objectives
The client's Rust application logic is enhanced to fetch the full, rich, live AI context from the AI Data Service.
A comprehensive library of highly specific and varied prompt templates is crafted for all 12 BunkerClasses and key Guard/Marketplace events.
The NAR generates narratives that are demonstrably deeply contextual, accurate to the live on-chain state, and engaging across a wide variety of player classes and items.
The NAR's FFI and core logic remain secure and robust.
Implementation Guidance
Action: Significantly enhance the Rust NAR ecosystem (C++ CC Client's Rust application logic and the NAR library) to generate rich, deeply contextual narratives for all Guard and Marketplace events. This requires leveraging the full, live, dynamic state now available from the functional Rust Indexer (via the AI Data Service). The focus is on narrative quality, specificity, and secure integration.
Implementation Details:
o ##### Context Acquisition (in Client's Rust App Logic - /client/rust-app-logic/src/nar/nar_prompt_builder.rs - Full Live Rich Context):
When Guard/Marketplace actions are confirmed (e.g., after a successful L3 transaction receipt + subsequent Indexer update signal), this logic is triggered.
It calls the functional Rust AI Data Service's GetAgentContextForNAR endpoint.
The context received is now a rich AIAgentInputDataContextProto reflecting the player's true, dynamic state from the live Indexer. This includes:
Their current, dynamically calculated BunkerClass (e.g., "Vanguard", not "Unclassed").
Their current, dynamically calculated ClassAffiliation (e.g., "Loyal").
Their full CoreStatsProto with all 12 live sub-stats.
A detailed list of equipped items with their specific GDD-defined names, rarities, traits, and origins.
Details of the specific item involved in the event (e.g., the "Echelon Starpath Array" that was just equipped).
The current NexusType (e.g., Guard Hangar, Marketplace).
o ##### Dynamic Prompt Engineering for NAR (in Rust App Logic - Templates for All 12 Classes, All Item Types, etc.):
Craft a comprehensive library of highly specific and varied prompt templates that leverage all available live context.
Example Templates: (As defined in original document for ItemEquipped (class change) and ExecuteTrade (buy)).
The prompt building function in Rust will now be significantly more sophisticated, selecting templates based on event type, player class, item type, etc., to create highly tailored prompts.
o Security: Continue to sanitize any string parts of the context (e.g., player names) before constructing the final prompt and passing it to the FFI.
o ##### Rust NAR FFI & llama.cpp Inference (Secure & Robust):
The secure FFI from P1 remains the same. The focus is on feeding it higher quality prompts.
Continue to enforce resource limits (timeouts) and robust error handling.
Testing:
Unit Tests (Rust App Logic): Test the advanced prompt construction logic with mock AIAgentInputDataContextProto objects representing a wide variety of player states.
Qualitative Evaluation (Critical - All 12 Classes & Rich Item Examples):
In the local Docker Compose simulation staging environment, use the Admin CLI and Guard/Marketplace UIs to create diverse player states (e.g., equip a full set of Corrupt Enforcer gear).
Trigger all relevant events (equip, unequip, buy, sell, level up).
Meticulously collect and review the generated NAR narratives.
Evaluate against a strict rubric: Does the narrative correctly identify the player's dynamic class and affiliation? Does it mention specific item names, rarities, and traits from the live data? Is the tone appropriate? Is there sufficient variety? Iterate on prompt templates in the client's Rust logic until the quality is consistently high.
Update docs/progress_logs/progress_phase_4.md:
Log the new, advanced prompt templates. Provide numerous examples of NAR outputs that demonstrate the use of full live dynamic context.
Design Rationale
Deeply contextual AI narratives are a key differentiator and a primary reward for player engagement. By making the NAR "aware" of the player's full, dynamic on-chain identity, we transform it from a gimmick into a core feature that makes the player's journey feel unique and recognized by the world itself. The client-side prompt engineering approach allows for rapid iteration on narrative quality without having to recompile the core C++ inference engine.
Operational Considerations
Local-First: The entire qualitative review and iteration loop will be performed within the local Docker Compose simulation, allowing for rapid changes to the Rust prompt templates and immediate testing.
Cloud-Ready: No changes are required for the NAR's cloud-readiness. The system is designed to function identically when the backend services it queries are deployed to the cloud.
Verification & Validation Criteria
A qualitative review by the AI Lead and Narrative Designer confirms that narratives are deeply contextual, accurate to the live dynamic on-chain state, and engaging across a wide variety of player classes and items.
Unit tests for the advanced prompt construction logic pass.
Testing Methodologies
The primary methodology for this task is qualitative evaluation. While unit tests will verify the logic, the ultimate success criteria is the quality of the generated text, which requires human review against a formal rubric.
Version Control Strategy
Branching: The enhanced NAR logic and prompts will be developed on a feature/nar-full-context branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead must re-review the prompt sanitization logic to account for the wider variety of dynamic data being included.
The resource limiting (timeout) and panic safety of the NAR FFI must be re-verified, as it will now be called much more frequently.
ReviewedBy: AI Lead, Narrative Designer, Client Lead (Rust), Rust NAR Lead, Security Lead.
ReviewOutcome: Approved for MVE Full Context Narrative Quality.
ValidationMethod: Qualitative review confirms that narratives are deeply contextual, accurate to the live dynamic L3 smart contract state, and engaging across a wide variety of player classes and items.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 4.7: Implemented Rust NAR Full Secure Contextual Narrative Generation (Guard/Marketplace, All Classes, Live Item/Stat/Nexus Details)." @Phase4/