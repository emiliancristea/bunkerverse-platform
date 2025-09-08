Task 5.3 (Revised): The SOCIAL Interface - Voice, Video, & Screensharing
(Secure Third-Party WebRTC Integration - Principles H, J, K, P, R)
Technical Reference
* gRPC/tonic Documentation
* WebRTC (Web Real-Time Communication) standards
* Chosen third-party RTC provider's SDK documentation (e.g., Twilio, LiveKit, Discord SDK)
* Finalized v0.1 API Contracts (rtc_service.proto)
* docs/ui/CLIENT_QML_UI_DEFINITION_MVE_V0.1.md
Context/Problem Statement
The SOCIAL interface, implemented in Phase 3, provides a robust foundation for community interaction with text chat and on-chain friends lists. However, to fully compete with and replace external platforms like Discord, the Control Center must offer high-bandwidth, real-time communication features like voice calls, video calls, and screensharing. Building a scalable, global WebRTC infrastructure from scratch is a massive and highly specialized undertaking. Therefore, this task focuses on securely integrating a professional, third-party RTC provider to deliver these features within our client.
Measurable Objectives
* A new Rust RTC Service is implemented to securely manage session tokens for the third-party provider.
* The Control Center Client's "SOCIAL" interface is enhanced with a fully functional UI for initiating and participating in voice/video calls.
* Users can successfully establish a one-on-one voice and video call with another user on their friends list.
* The integration is secure, preventing unauthorized access to communication channels.
Implementation Guidance
Action: Build out the advanced, real-time communication suite in the SOCIAL interface by integrating a professional third-party WebRTC service.
* New Rust RTC Service (/services/rtc/):
o API Definition (rtc_service.proto): Define a simple gRPC service with one primary RPC: GenerateRtcSessionToken(GenerateRtcSessionTokenRequest).
o Third-Party Integration:
* This service will contain the securely stored API keys for our chosen RTC provider (e.g., Twilio).
* The GenerateRtcSessionToken RPC will authenticate the player's BUNKERVERSE JWT.
* It will then use the provider's admin SDK to generate a short-lived, permissioned session token that grants the specific user access to a specific "room" or channel.
* It returns this ephemeral token to the client.
* Client Implementation (Rust & QML):
a. Rust Application Logic (/client/rust-app-logic/src/social/rtc_handler.rs):
1. Integrate the native SDK of the chosen RTC provider.
2. Implement #[cxx_qt::qinvokable] functions like initiateVoiceCall(friend_id).
3. This function will first call our Rust RTC Service to get a session token.
4. It will then use the provider's SDK to connect to the RTC session using that token.
5. It will handle all the SDK's callbacks for connection status, remote user joining, media streams starting, etc., and expose this state to the QML layer via signals and properties.
b. QML Frontend (/client/qml-ui/interfaces/social/):
* UI Enhancements: Add "Call" and "Video Call" icons to the friends list and chat interface.
* Call UI: Implement a new CallView.qml component. This will be a floating or docked window that shows the avatars of users in the call, with controls for mute, deafen, enable/disable video, and hang up.
* The QML components will render video streams and react to state changes (e.g., showing a "connecting..." status) by binding to the properties and signals exposed by the Rust rtc_handler.
Design Rationale
Building a global, low-latency, and secure WebRTC infrastructure is a domain-specific, multi-million dollar problem. Leveraging a specialized third-party provider is the only pragmatic and strategically sound approach. It allows us to deliver a world-class communication experience without diverting years of R&D from our core mission. The RTC Service acts as a secure broker, ensuring our master API keys for the provider are never exposed to the client, and that all session access is authenticated and authorized through our own JWT system.
Operational Considerations
* Local-First: All development will be done against the third-party provider's sandbox or developer environment. The new RTC Service will be added to the local Docker Compose simulation.
* Cloud-Ready: The RTC Service is a stateless, containerized service ready for cloud deployment. Its provider API keys will be loaded from AWS Secrets Manager in production.
* Cost: The third-party RTC provider will be a significant operational cost that scales with usage (typically per minute per user). This must be factored into the MVE's operational budget.
Verification & Validation Criteria
* Two users, running the client in the local development environment, can successfully initiate and conduct a clear, low-latency voice and video call with each other.
* An unauthorized user (without a valid JWT) attempting to get a session token from the RTC Service is correctly rejected.
Testing Methodologies
* Unit Tests: For the RTC Service token generation logic.
* Integration Tests: The Rust client logic will be tested to ensure it can correctly obtain a token from the RTC Service and initiate a connection with a mock/stubbed version of the third-party SDK.
* E2E Tests (Manual): This feature requires manual end-to-end testing with two instances of the Control Center client to validate call quality, connection stability, and the full user experience.
Version Control Strategy
* Branching: The voice/video feature set will be developed on a feature/social-interface-rtc branch.
* Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
* A mandatory security review of the RTC Service is required, focusing on the authentication and authorization logic to ensure only valid users can obtain session tokens for channels they are permitted to join.
* The client-side integration of the third-party SDK must be reviewed to ensure it is configured securely and does not expose sensitive user data.
ReviewedBy: Client Lead (Rust & QML), Rust Backend Lead, UI/UX Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All unit and integration tests pass. Manual E2E tests confirm that users can successfully conduct voice and video calls within the client.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 5.3: Implemented SOCIAL Interface (Voice, Video & Screensharing via Secure Third-Party Integration)." @Phase5/

------------------------------------------------------------------------------------------------------------------
