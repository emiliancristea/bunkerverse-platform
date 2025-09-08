Task 1.2: Service Persistence Layer Foundation
(Secure Storage with Redb for Backend Services, AES-256 Encryption - Principles G, I, J, N, R)
Technical Reference
redb crate documentation
aes-gcm and rand crate documentation
prost crate documentation
Finalized v0.1 Protobuf Schemas
Context/Problem Statement
While our L3 Smart Contracts serve as the canonical source of truth for core player state, our off-chain Rust services (like the Mission Service, Forum Service, etc.) require their own persistent storage for tracking progress, managing sessions, and storing application-specific data. This task implements a foundational, secure, and high-performance storage layer using Redb, an embedded database. This layer will be used by various backend services in later phases. It includes robust, validated AES-256 encryption for all data at rest to ensure the security of user and application data.
Measurable Objectives
A reusable Rust module for a secure storage adapter using Redb is implemented and tested.
The adapter provides functional, atomic event/snapshot operations for service-specific data.
A robust, validated AES-256-GCM envelope encryption service is implemented.
All new code passes a mandatory, detailed security code review focusing on the encryption implementation.
Implementation Guidance
Action: Implement a foundational Rust storage layer for backend services in /libs/common-rust/src/storage/. This implementation will use Redb as the chosen MVE local database technology (validated in P0.2 PoC) and will support all finalized P0.3 schemas as needed for service-specific state. It must include robust, validated AES-256 encryption for all data at rest.
Storage Adapter (Rust Trait & Redb Implementation - Functional):
In /libs/common-rust/src/storage/adapter.rs, define the IServiceStorageAdapter Rust trait:
code
Rust
#[async_trait]
pub trait IServiceStorageAdapter {
    // Appends a batch of records for a specific key atomically.
    async fn append_records(...) -> Result<...>;

    // Retrieves records for a specific key.
    async fn get_records(...) -> Result<...>;

    // Stores a complete state snapshot.
    async fn store_snapshot(...) -> Result<...>;

    // Retrieves the most recent state snapshot.
    async fn get_latest_snapshot(...) -> Result<...>;
}
In /libs/common-rust/src/storage/redb_adapter.rs, implement RedbStorageAdapter which implements the trait.
Redb Data Model:
Redb will manage several tables (logical collections) as defined by the consuming service (e.g., for Mission Service: mission_progress_table).
The implementation will use Redb's transactional write API to ensure atomicity.
Data Serialization:
Use the prost crate for all serialization/deserialization of Protobuf messages.
Encryption (AES-256-GCM at Rest - Functional & Validated - Principle R):
Implement an EncryptionService in Rust. This service will be initialized with a Key Encryption Key (KEK) loaded securely from configuration (which in turn loads from the local .env file for the local Docker Compose simulation).
Encryption Logic:
When encrypting a data blob (a serialized Protobuf message), the EncryptionService will:
Generate a new, cryptographically random Data Encryption Key (DEK) and a nonce for AES-256-GCM.
Encrypt the data blob with the DEK and nonce.
Encrypt the DEK itself using the master KEK (envelope encryption).
Return a combined, structured EncryptedBlob containing (encrypted_dek, nonce, ciphertext). This is what's stored as the value in Redb.
Decryption Logic:
When decrypting an EncryptedBlob:
The EncryptionService decrypts the encrypted_dek using the master KEK.
It then uses the decrypted DEK and the stored nonce to decrypt the ciphertext, which returns the original serialized Protobuf Vec<u8>.
This ensures each piece of data is encrypted with a unique key (DEK), improving security.
Error Handling:
Define a comprehensive StorageError enum with variants for IoError(redb::Error), SerializationError, EncryptionError, etc.
Security Code Review:
A mandatory, detailed peer review of all new Rust code for this task, with a specific focus on:
Correctness of the EncryptionService implementation (nonce reuse, key handling, correct use of aes-gcm crate).
Secure loading and handling of the master KEK.
Atomicity of Redb transactions.
Secure database file permissions if a service creates the Redb file itself.
Testing (Rust):
Unit Tests:
Test the RedbStorageAdapter methods against a temporary in-memory or on-disk Redb database.
Exhaustively test the EncryptionService to ensure data can be encrypted and decrypted correctly, and that tampered ciphertext or incorrect keys lead to decryption failures.
Integration Tests: N/A for this library; will be tested via consuming services in later phases.
Update docs/progress_logs/progress_phase_1.md:
Log the implementation details of the RedbStorageAdapter.
Detail the EncryptionService logic, including the envelope encryption scheme and KEK management strategy for the MVE.
Summarize the database schema pattern for Redb.
Document adherence to First Principles G, I, J, N, R.
Log the outcome of the mandatory security code review for the encryption and storage logic.
Design Rationale
While the L3 is the canonical source of truth for assets, individual backend services require their own persistent storage for operational data that is not globally significant (e.g., mission progress tracking before a reward is granted). Creating a generic, secure, and reusable storage library with built-in encryption (IServiceStorageAdapter) is a core application of the DRY (Don't Repeat Yourself) principle. It ensures that all services use the same high standard of data security and persistence without re-implementing the logic. Redb is chosen as it is a high-performance, embedded Rust database, perfectly suited for our local-first development environment.
Operational Considerations
Local-First: Each service using this storage adapter will have its own Redb database file, managed within its Docker container in the docker-compose.yml setup. This ensures data isolation between services.
Cloud-Ready: While Redb is excellent for local dev and certain use cases, services requiring high availability in the cloud (like the Mission Service) will switch their storage adapter implementation to use a managed database like AWS RDS (as defined in the IaC). The use of the IServiceStorageAdapter trait ensures this switch can be made with minimal code changes to the service's business logic. The EncryptionService will switch from loading its KEK from a local .env file to loading it from AWS Secrets Manager.
Verification & Validation Criteria
All unit and integration tests pass for the storage library.
The EncryptionService is proven to be secure and correct through exhaustive testing.
The library's implementation is successfully used by the Identity Service (Task 1.1) for its persistent data needs.
The mandatory security code review is completed and signed off.
Testing Methodologies
Unit Tests: Will cover all methods of the RedbStorageAdapter and EncryptionService against a temporary Redb database.
Integration Tests: The functionality will be implicitly tested through the integration tests of the services that consume this library.
Version Control Strategy
Branching: The library will be developed on a feature/ branch (e.g., feature/secure-storage-adapter).
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The EncryptionService implementation is one of the most critical pieces of security-sensitive code in the entire backend. It requires mandatory, meticulous review and sign-off from the Security Lead.
Correct handling of the master KEK is paramount.
ReviewedBy: Rust Backend Lead, Security Lead, Lead Architect.
ReviewOutcome: Approved.
ValidationMethod: All unit tests pass for the storage adapter and encryption service. Security review passed.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 1.2: Implemented Secure Service Persistence Layer Foundation (Redb Adapter, AES-256 Envelope Encryption)." @Phase1/