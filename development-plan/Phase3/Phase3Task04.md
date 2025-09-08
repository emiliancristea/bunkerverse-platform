Task 3.4 (Revised & Expanded): The SOCIAL Interface - Chat & Friends Management
(Functional Real-Time Chat Service & Client UI with On-Chain Friends List - Principles H, J, K, P, R)
1. Technical Reference
API Contracts (v0.1): social_service.proto, chat_service.proto (conceptual for WebSocket)
L3 Smart Contracts: BunkerguardManager.sol ABI (for addSocialConnection)
Libraries: gRPC/tonic, sqlx, tokio-tungstenite (for WebSockets), serde, CXX-Qt, QML.
UI/UX Design Document: docs/ui/CLIENT_Q..._MVE_V0.1.md (for the SOCIAL Interface and its subpages).

Library/Tool Versions:
| Library/Tool | Version | Notes |
|--------------|---------|-------|
| Qt SDK | 6.9.2 (MSVC 2022 64-bit) | Path: D:\DOCUMENTS\Qt\6.9.2\msvc2022_64 |
| cxx-qt Crate | 0.7.2 | The crate aims to support all Qt 6 versions; pinned for stability. |
| rustc | 1.80.0 (Stable) | Pinned via rust-toolchain.toml. |
2. Context/Problem Statement
Context: The Archives (Task 3.3) provides a crucial hub for asynchronous, persistent communication. However, to create a truly vibrant community and replace external tools like Discord, the Control Center needs a robust, integrated, real-time communication suite.
Problem: Implementing a secure, scalable real-time chat system is a complex undertaking. It requires a new backend service capable of managing persistent WebSocket connections, handling message broadcasting, and ensuring all user-generated content is sanitized. Furthermore, the client UI must be re-architected to handle this new real-time communication paradigm. This task focuses on building the foundational layer of the "SOCIAL" interface, including friends list management (which interacts with the on-chain social graph) and a secure, real-time text chat service.
3. Measurable Objectives
Backend:
a. A new, functional Rust Chat Service capable of managing authenticated WebSocket connections and broadcasting messages is deployed to the local Docker Compose simulation.
b. A new, functional Rust Social Service is implemented to manage on-chain friend relationships.
Client (QML & Rust):
o The "SOCIAL" interface and its subpages (ChatHomePage.qml, ChatInterface.qml, ChatsPage.qml) are fully implemented.
o The client can establish a persistent, secure WebSocket connection to the Chat Service.
o Users can manage their on-chain friends list and engage in real-time, one-on-one text chat.
End-to-End Validation: Two users can successfully become friends on-chain, initiate a private chat, and exchange real-time messages, all within the Control Center.
4. Implementation Guidance
New Rust Chat Service (/services/chat/):
o Architecture: This will be a WebSocket server built using tokio-tungstenite and Axum. It is a stateful service, maintaining a HashMap of connected users (PlayerId -> WebSocket Sink).
o Authentication: When a client initiates a WebSocket connection, the upgrade request must contain a valid platform JWT. The service will validate this JWT before upgrading the connection to a WebSocket.
o Message Handling: The service will listen for incoming JSON messages from clients (e.g., { type: "PRIVATE_MESSAGE", to: "PlayerID", content: "..." }).
o Routing & Broadcasting: Upon receiving a message, the service will perform security checks (see below), find the recipient's WebSocket connection in its HashMap, and forward the message.
o Security (Critical): All incoming content fields must be rigorously sanitized on the backend before being forwarded to any other client to prevent XSS attacks. Implement rate limiting to prevent spam.
New Rust Social Service (/services/social/):
API Definition (social_service.proto): Define gRPC RPCs: GetFriendsList, SendFriendRequest, AcceptFriendRequest, RemoveFriend.
L3 Integration (AcceptFriendRequest): This function is the core of on-chain social connections. It will construct and submit a addSocialConnection transaction to the L3 Smart Contracts for both the sender and receiver, creating a permanent, on-chain record of the friendship.
Indexer Interaction (GetFriendsList): This function will query the live Indexer to fetch the user's list of on-chain connections (SocialConnectionAdded events).
Rust Application Logic (/client/rust-app-logic/src/social/):
o State Management: In the rustBackend QObject, create new cxx_qt::qproperty models: friendsModel (a list of friends), and activeChatsModel (a model for current conversations).
o WebSocket Client: Implement the logic to establish and maintain a persistent, authenticated WebSocket connection to the Chat Service. Handle incoming messages and route them to the correct chat state.
o gRPC Client: Implement the client logic for the Social Service.
o Handlers: Implement #[cxx_qt::qinvokable] functions: fetchFriendsList, sendFriendRequest, acceptFriendRequest, and sendChatMessage(recipientId, messageContent). The sendChatMessage function will send the message over the live WebSocket connection.
QML Frontend (/client/qml-ui/interfaces/social/):
o ChatHomePage.qml (The Main/Home Page):
Layout: A welcoming page for the social hub. It might display pending friend requests, a summary of recent conversations, and a prominent "Add Friend" button.
Backend Interaction: On load, it calls rustBackend.fetchFriendsList() and other relevant summary functions.
o ChatsPage.qml (List of Chats):
Layout: A left-side panel, similar to Discord or Slack. It will contain a ListView that binds to the rustBackend.friendsModel. Each delegate represents a friend, showing their name, avatar, and online status.
Backend Interaction: Clicking a friend in the list will be the primary way to open or navigate to a ChatInterface for that specific conversation.
o ChatInterface.qml (The Chat View):
Layout: The main chat area. It will have a header with the friend's name, a central ListView to display the message history for that conversation, and a TextArea at the bottom for input.
Backend Interaction: The ListView of messages will bind to a dynamic model in the rustBackend for the active chat. When the user types a message and hits enter (or a "Send" button), it will call rustBackend.sendChatMessage(activeFriendId, messageText). New incoming messages from the WebSocket will be automatically added to the model by the Rust backend, causing the UI to update in real-time.
5. Design Rationale
Building a native, real-time chat and social system is critical for achieving user retention and a self-sustaining community. By making friendships an on-chain action, we elevate them to a persistent, canonical part of a player's identity. Separating the stateful, real-time chat logic into its own WebSocket-based service is a standard, scalable pattern, while the less time-sensitive social graph management (adding friends) can be handled by a standard gRPC service.
6. Operational Considerations
Local-First: The new Chat Service and Social Service will be added to the docker-compose.yml file.
Cloud-Ready: The Chat Service is a stateful service (managing active WebSocket connections) and will require careful configuration in Kubernetes for scaling, potentially using a sticky session configuration on the load balancer to ensure a user's messages are routed to the correct instance.
7. Verification & Validation Criteria
A user can launch the client, switch to the SOCIAL interface, and see their friends list.
Two users can successfully send and receive real-time messages.
A user can send a friend request, the other can accept, and this triggers a verifiable transaction on the L3.
8. Testing Methodologies
Unit Tests: For chat message sanitization logic and Social Service business logic.
Integration Tests: Run the full social backend in the local Docker Compose simulation. Use a test harness to simulate multiple clients connecting via WebSockets, sending messages, and forming friendships.
E2E Tests: A manual test involving two instances of the Control Center Client, where two users log in and successfully chat and become friends.
9. Version Control Strategy
Branching: This feature set will be developed on a feature/social-interface-chat branch.
Commits: The Git Commit message for this task will be exactly as specified.
10. Security Audit & Compliance Checkpoints
A mandatory, exhaustive security audit of the Chat Service is required. Focus is on input sanitization (preventing XSS), authentication of WebSocket connections, and rate limiting/anti-spam.
The Social Service's interaction with the L3 smart contracts must be reviewed to prevent exploits.
ReviewedBy: Client Lead (Rust & QML), Rust Backend Lead, L3 Smart Contracts Lead, UI/UX Lead, Security Lead.
ReviewOutcome: Approved.
ValidationMethod: All tests pass. Users can manage friends and communicate via real-time text chat, with friendships being recorded on-chain.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 3.4: Implemented SOCIAL Interface (Friends Management & Real-Time Text Chat)." @Phase3/