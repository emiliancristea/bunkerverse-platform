Task 3.3 (Revised & Expanded): THE NEXUS - "ARCHIVES" (Forum & Knowledge Base) Implementation
(Functional Multi-Section Forum with Secure Backend & L3 Rewards - Principles H, J, K, R)
1. Technical Reference
API Contracts (v0.1): forum_service.proto
L3 Smart Contracts: NTC.sol (for transfer), BunkerguardManager.sol (for on-chain state) ABIs.
UI/UX Design Document: docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md (specifically the sections for THE NEXUS -> ARCHIVES and all its subpages).
Libraries: gRPC/tonic, sqlx, pulldown-cmark (for Markdown parsing), ammonia (for HTML sanitization), CXX-Qt, QML.

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
2. Context/Problem Statement
Context: Following the establishment of the core UI shell and the economic hub (The Plaza), we must build the intellectual and strategic heart of the community.
Problem: A thriving community requires a dedicated, in-client space to create and share persistent knowledge, such as game guides, lore theories, and feedback. Using external platforms like Discord for this is inefficient, as valuable information is lost in the noise of real-time chat. The "ARCHIVES" is the solution-a fully-featured, integrated forum and knowledge base that captures and organizes this collective intelligence, and even rewards valuable contributions on-chain.
3. Measurable Objectives
Backend: A new, functional Rust Forum Service with a persistent MySQL database is implemented and deployed to the local Docker Compose environment.
API: The Forum Service's gRPC API supports the full lifecycle of forum content: creating and fetching categories, threads, and posts.
Client (QML & Rust): The full ARCHIVES UI and all its functional subpages (ArchivesPage, GuidesPage, LorePage, ThreadsPage, etc.) are implemented, replacing the placeholder.
End-to-End Validation: The full forum lifecycle is validated: a user can create a thread and post, another user can reply, and an admin can reward a high-quality post with an on-chain NTC transaction.
Security: All user-generated content is rigorously sanitized to prevent XSS attacks.
4. Implementation Guidance
API Definition (forum_service.proto):
o Finalize the gRPC service. Define RPCs like GetCategories(category_type), GetThreads(category_id), GetPosts(thread_id), CreateThread, CreatePost, and the admin-only RewardPost(post_id, amount, reason).
o Define enums for CategoryType { GUIDES, LORE, FEEDBACK, GENERAL }.
Database Schema (MySQL via sqlx-cli):
o Create migrations for categories (with a category_type field), threads (with is_stickied, is_locked flags), and posts tables. The posts table should store both the raw Markdown input and the sanitized HTML output.
Service Logic (Axum/Tonic):
o Content Creation (CreateThread, CreatePost):
These RPCs will receive user-generated content in Markdown format.
Markdown Parsing & Sanitization (Critical Security): Upon receiving a post, the service will:
a. Use a crate like pulldown-cmark to parse the Markdown into HTML.
b. Use a crate like ammonia with a strict whitelist of allowed tags (<b>, <i>, <ul>, <li>, <a>, etc.) and attributes (href) to rigorously sanitize the generated HTML. This is a non-negotiable step to prevent XSS.
c. Store both the original Markdown and the sanitized HTML in the database.
o Content Fetching: The Get... RPCs will query the MySQL database and return the pre-sanitized HTML for rendering in the client.
o L3 Reward Integration (RewardPost):
This admin-only RPC will, upon validation, construct and submit an NTC transfer transaction to the L3 via the Transaction Submission Service, rewarding the post's author.
Rust Application Logic (/client/rust-app-logic/src/nexus/archives_handler.rs):
o State Management: Create new cxx_qt::qproperty models in the rustBackend for categoriesModel, threadsModel, postsModel.
o Handlers: Implement #[cxx_qt::qinvokable] functions to interact with the new Forum Service (fetchCategories(category_type), createPost(thread_id, content), etc.).
QML Frontend (/client/qml-ui/screens/nexus/archives/):
o ArchivesPage.qml (The Main Hub Page):
Layout: This is the main landing page for the Archives. It will feature large clickable cards or a navigation rail for the main sections: "Guides," "Lore," "Feedback," and "General Discussions." It could also feature a "Latest Posts" or "Hot Threads" ListView.
Backend Interaction: On Component.onCompleted, it can pre-fetch summary data, e.g., rustBackend.fetchLatestThreads().
o GuidesPage.qml, LorePage.qml, FeedbackPage.qml, ThreadsPage.qml (The Category Views):
Layout: These pages will be structurally similar, each containing a ListView of threads for their specific category. The view will show thread titles, author, reply count, and view count. A prominent "Create New Thread" button will be present.
Backend Interaction: On load, each page calls rustBackend.fetchThreads(category_type), where category_type is specific to the page (e.g., GUIDES). The ListView will bind to rustBackend.threadsModel.
o Thread View (ThreadView.qml):
Layout: A view that displays the content of all posts in a thread, paginated. The first post is displayed prominently. A "Reply" TextArea is at the bottom.
Backend Interaction: This view is pushed onto the navigation stack when a user clicks a thread. It receives the thread_id as a property and calls rustBackend.fetchPosts(thread_id). The reply TextArea is connected to a "Submit" button that calls rustBackend.createPost(thread_id, post_content).
Content Rendering (Security): The post content will be rendered using a QML Text element with textFormat: Text.RichText. It will be bound to the pre-sanitized HTML provided by the backend, never the raw user input.
5. Design Rationale
A native, in-client forum is a critical pillar for building a self-sustaining community knowledge base. By structuring it into distinct, curated sections (Guides, Lore, etc.), we improve content discovery. Performing all Markdown parsing and HTML sanitization on the backend is a critical security pattern that ensures no malicious, unsanitized content ever reaches any client. The on-chain NTC reward mechanism directly incentivizes the creation of high-value community content, a powerful flywheel for engagement.
6. Operational Considerations
Local-First: The new Forum Service and its MySQL database will be added to the docker-compose.yml file.
Cloud-Ready: The Forum Service is a containerized, stateless service ready for scaling. The MySQL database will be provisioned as a managed AWS RDS instance.
Moderation: This feature requires human moderation. Initial tools can be admin flags in the database (e.g., is_hidden, is_stickied) managed via the Admin CLI.
7. Verification & Validation Criteria
A user can successfully navigate to THE NEXUS -> ARCHIVES and browse all subpages.
A user can create a thread and a post using Markdown, and it is rendered correctly and securely for another user.
An administrator can reward a post, triggering a verifiable NTC transaction on the L3.
8. Testing Methodologies
Unit Tests: For the Forum Service's business logic, and especially for the Markdown parsing and HTML sanitization logic.
Integration Tests: Run the Forum Service and its DB in the local Docker Compose simulation and test the full gRPC API.
E2E Tests: A manual test where User A creates a post, User B replies, and an Admin rewards the post.
Security Testing: Dedicated testing to try and bypass the HTML sanitization filter to achieve XSS.
9. Version Control Strategy
Branching: This feature set will be developed on a feature/nexus-archives-forum branch.
Commits: The Git Commit message for this task will be exactly as specified.
10. Security Audit & Compliance Checkpoints
A mandatory, exhaustive security audit of the user-generated content pipeline is required. The primary focus is on the backend HTML sanitization to prevent XSS. This is the most critical security checkpoint of this task.
Access control for all admin/moderator RPCs must be rigorously tested.
ReviewedBy: Client Lead (Rust & QML), Rust Backend Lead, L3 Smart Contracts Lead, UI/UX Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All tests pass. A user can fully interact with the forum system, content is sanitized, and the L3 reward mechanism is functional and secure.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.3: Implemented THE NEXUS - ARCHIVES (Functional In-Client Forum)." @Phase3/