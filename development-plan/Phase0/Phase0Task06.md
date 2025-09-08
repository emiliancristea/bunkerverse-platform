## Task 0.6: Configuration Management Pattern Implementation
(Finalized, Integrated, Secure Defaults, Secrets Management, & Dual-Mode Flag Propagation - Principle M, R)

### Technical Reference
 figment or config-rs crate documentation
 Docker Compose .env file documentation
 docs/PROJECT_GOVERNANCE_AND_WORKFLOWS.md (Security Charter)

### Context/Problem Statement
Hardcoding configuration values (like API endpoints, port numbers, or feature flags) directly into the application code is insecure, inflexible, and leads to disastrous deployment errors. We need to establish a consistent, secure, and layered pattern for managing configuration across all backend services and the client. This pattern must support our Local-First, Cloud-Ready doctrine by working seamlessly in both the local Docker Compose environment and being prepared for a production-grade secrets management solution like AWS Secrets Manager.
### Measurable Objectives
 A finalized, layered configuration strategy is implemented in the "smart stubs" for both backend services and the client's Rust logic.
 A successful demonstration of loading defaults from a file, overriding them with environment variables, and consuming secrets from a local .env file.
 A formal security review of all default configuration files is completed and signed off, confirming they are secure by default.
 The enable_crypto and show_crypto flags are correctly propagated and defaulted to false.

### Implementation Guidance
Action: Finalize and implement a consistent configuration management strategy across all components (Rust backend services, Rust client application logic, C++ client shell if needed, post-MVE C++ game server stubs). This strategy must prioritize secure defaults, robust secrets management, and clear, secure propagation and usage of dual-mode flags (enable_crypto, show_crypto).

#### Configuration Strategy & Implementation:
 Rust Backend Services (Indexer, Marketplace/Axum, Identity, etc.):
 Utilize a layered configuration approach with a crate like figment or config-rs.
 Default Configuration: A default.toml file committed to Git for each service, containing all non-sensitive configuration parameters with secure and sensible defaults (e.g., log levels set to INFO, enable_crypto: false).
 Environment Variables: Allow overriding any configuration parameter via environment variables (highest precedence). This is the primary way Docker Compose will pass configuration.
 Secrets Management:
 Local Simulation: For the local Docker Compose environment, all secrets (database passwords, API keys, JWT signing keys, enable_crypto flag) will be managed via a .env file at the root of the project, which is gitignored. Docker Compose will inject these as environment variables into the appropriate service containers.
 Cloud-Ready: The configuration loader will be designed to seamlessly switch to loading these secrets from a service like AWS Secrets Manager when deployed to a cloud environment, with no code changes required.
 Settings Structs: Each Rust service will define a strongly-typed Settings struct (using Serde for deserialization) that aggregates all configuration.
 Control Center Client (Rust Application Logic & C++ Shell/QML):
 Rust Application Logic (/client/rust-app-logic): Use figment or config-rs similarly to backend services. A default_client_config.toml can provide defaults. enable_crypto and show_crypto flags will be loaded here. For MVE, show_crypto will default to false and enable_crypto will also be false.
 QML UI: The Rust application logic will expose configuration values (like show_crypto, API endpoints) to the QML layer via CXX-Qt properties. QML itself will not directly load config files.
 C++ Qt Shell (/client/cpp-shell): If the C++ shell needs specific configuration not managed by Rust (unlikely for MVE core logic), it can use QSettings or a simple config file, but primary configuration should be Rust-driven.
 Post-MVE C++ Game Server Stubs: Use a simple config file (e.g., JSON or INI) or environment variables for basic settings (e.g., service endpoints).
 Dual-Mode Flag Handling:
 enable_crypto (for backend services and Rust client app logic): Controls whether crypto-related logic paths (e.g., NTC staking, non-custodial wallet ops) are active. Defaults to false. In a future cloud production environment, this will be a securely managed secret.
 show_crypto (for Rust client app logic, exposed to QML): Controls whether UI elements related to on-chain features are visible. Defaults to false.
 Ensure clear documentation on how these flags affect each component's behavior.
#### 0.6.a. Security Review of All Default Configurations (Rigorous - Principle R):
 Action: Conduct a thorough security review of all default.toml (or equivalent) configuration files for every service and the client.
 Checklist:
 Are all debug options/endpoints disabled by default?
 Are logging levels appropriate for production (e.g., INFO, not DEBUG)?
 Are default timeouts for network requests reasonable and secure?
 Are default rate limits set to sensible, non-exploitable values?
 Are security headers for any HTTP services configured with secure defaults?
 Is enable_crypto and show_crypto set to false by default in all relevant components?
 Are there any hardcoded placeholder credentials or weak default secrets? (Must be none).
 Are file paths for data storage secure by default?
 Are default permissions for created files/directories secure?
 Document review findings and any changes made to defaults in docs/progress_logs/progress_phase_0.md.
#### Update docs/progress_logs/progress_phase_0.md:
 Log the chosen configuration libraries/methods for each component type.
 Provide examples of Settings structs and default configuration files for a key Rust service and the Rust Client App Logic, showing how dual-mode flags are handled.
 Document the outcome of the rigorous default configuration security review.
 Detail the .env file structure for the local Docker Compose simulation and explain how it maps to the future AWS Secrets Manager integration.
### Design Rationale
A layered configuration approach provides maximum flexibility for different environments (local, dev, cloud) without compromising security. Externalizing all configuration and secrets is a cornerstone of the Twelve-Factor App methodology and is essential for a Cloud-Ready architecture. Using a simple .env file for local development perfectly simulates a production secrets management system without adding unnecessary complexity to the local setup.
### Operational Considerations
 Local-First: The .env file will be the single source of truth for all secrets and environment-specific settings during local development. A .env.example file will be committed to the repository to guide developers.
 Cloud-Ready: The configuration strategy is designed for a seamless transition. The parallel-developed Terraform scripts will provision secrets in AWS Secrets Manager. The Kubernetes manifests will mount these secrets into the pods as environment variables. No application code will need to change to move from the local .env file to the production secrets manager.
### Verification & Validation Criteria
 Successful demonstration of configuration loading in a Rust service stub running in Docker Compose, correctly picking up defaults from default.toml, overrides from the .env file, and secrets from the .env file.
 enable_crypto: false is confirmed as the default active setting for all components.
 The default configuration security review is completed and signed off by the Security Lead.
### Testing Methodologies
Unit tests will be written for any complex configuration parsing logic. Integration tests will validate that services running in Docker Compose start correctly with the provided environment variables and connect to other services using the configured DNS names.
### Version Control Strategy
 Branching: Configuration logic is developed on feature/ branches.
 Commits: All default.toml and .env.example files are committed to the repository. The actual .env file is added to .gitignore and never committed. The Git Commit message for this task will be exactly as specified.
### Security Audit & Compliance Checkpoints
 The security review of all default configurations is a mandatory checkpoint.
 The secrets management strategy (using .env locally and preparing for AWS Secrets Manager) must be reviewed and approved by the Security Lead.
ReviewedBy: Lead Rust Developer, Client Lead, DevOps Lead, Security Lead.
ReviewOutcome: Secure Configuration Management Pattern (Dual-Mode Aware, AWS Secrets Manager Integrated) Approved for Phase 0.
ValidationMethod: Successful demonstration of configuration loading in a Rust service stub and the Rust Client App Logic stub, correctly picking up defaults, overrides from environment variables, and secrets from a local file equivalent for Docker Compose. enable_crypto: false is confirmed as the default active setting.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 0.6: Finalized Secure Configuration Management Pattern (Dual-Mode Aware, Figment/Config-rs, Local .env Secrets for Stubs), Defaults Hardened." @Phase0/

------------------------------------------------------------------------------------------------------------------
