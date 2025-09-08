Task 0.5: Initial IaC & Docker Compose Setup
("Smart Stubs" for Arbitrum Orbit L2, Axum Marketplace, etc. with Security Baseline, Secure Defaults, & Dual-Mode Configuration - Principles A, M, R)
Technical Reference
Docker Compose Documentation
Terraform Documentation
Kubernetes Documentation (minikube or k3d for local simulation)
Arbitrum Orbit Local Development Setup Guides
docs/SMART_STUBS_SPECIFICATION.md
Context/Problem Statement
With our CI pipelines in place, developers need a way to run the entire distributed backend on their local machines for rapid development, testing, and debugging. Furthermore, to adhere to our "Cloud-Ready" doctrine, we must develop our production infrastructure definitions in parallel with our application code. This task creates the foundational local development environment using Docker Compose and initiates the creation of our cloud infrastructure using Terraform, deploying "smart stub" versions of all services to both.
Measurable Objectives
A functional docker-compose.yml that can start the entire "smart stub" backend, including a simulated Arbitrum Orbit L2 network, with a single command.
Initial Terraform scripts that can provision a secure, baseline cloud development/testing environment are created and validated locally (e.g., against minikube).
All deployed stubs and infrastructure (both local and in IaC) adhere to a documented security baseline (least privilege, network isolation).
Stubs are proven to be configurable for dual-mode behavior.
Implementation Guidance
Action: Create functional Docker Compose configurations for local development and initial Terraform scripts for a cloud-ready development/testing environment. These setups will deploy "smart stub" versions of all backend services (including a simulated Arbitrum Orbit L2 sequencer network, Indexer, Axum Marketplace, Identity, AI Data, etc.), rigorously applying security best practices from the start (secure defaults, least privilege, network isolation) and ensuring stubs can be configured for dual-mode behavior.
"Smart Stub" Implementation & Dockerization (Securely Built, Reflecting Chosen Tech):
For each backend Rust service (Indexer, Marketplace, Identity, AI Data, Payment, Mission, Social, Feedback), develop a "smart stub" version. These stubs:
Implement the gRPC/OpenAPI interfaces defined in P0.3.
Return hardcoded, schema-valid responses that reflect the full MVE lore/mechanics.
Adhere to docs/SMART_STUBS_SPECIFICATION.md, including configurable latency/error simulation and behavior changes based on the enable_crypto environment variable.
Provide structured JSON logging of requests and responses.
Arbitrum Orbit L2 Stub Specifics:
The local simulation will run a full multi-node Arbitrum Orbit sequencer network as defined in the official documentation, configured to use a local Ethereum node (e.g., Anvil/Hardhat). This will be orchestrated within Docker Compose.
Marketplace Service Stub (Axum-based):
Implement Axum route handlers for defined gRPC/OpenAPI endpoints.
Simulate Redis caching behavior.
IPFS Stub: A simple HTTP server mimicking basic IPFS gateway GET /ipfs/<CID> behavior.
Dockerfiles for Stubs (/infra/dockerfiles/[service_name]/Dockerfile):
Use minimal, trusted base images (e.g., rust:alpine for build stage, alpine or gcr.io/distroless/static-debian11 for runtime).
Compile Rust stubs in a builder stage. Copy only the compiled binary to the final stage.
Ensure services run as a non-root user.
Set file system permissions to be as restrictive as possible.
Dockerfiles must be scanned by Trivy in the CI/CD pipeline (Task 0.4).
Local Development Environment (/docker-compose.yml):
Define services for all backend smart stubs (Arbitrum Orbit Sequencer(s), local ETH node, Indexer, Marketplace, Identity, etc., and mocks for Arweave/IPFS).
Include services for necessary databases/caches (Redis, Elasticsearch/Typesense).
Define an explicit Docker network (e.g., bunkerverse-dev-network) for inter-service communication. All inter-service communication must use DNS service names (e.g., http://identity-service:50051).
Services should listen on 0.0.0.0 within their containers. Expose ports to the host machine only as necessary.
All default credentials for databases must be configurable via environment variables from a .env file (which is gitignored).
Pass enable_crypto and show_crypto environment variables to relevant stubs.
Cloud-Ready Infrastructure (IaC developed in parallel - /infra/terraform/):
VPC & Networking: Define a VPC with public/private subnets, NAT Gateways, Internet Gateway.
EKS Cluster (or other Kubernetes): Provision a managed Kubernetes cluster. Define node groups.
Container Registry: Set up AWS ECR for storing Docker images of stubs.
Managed Databases/Services: Define modules for RDS (MySQL), ElastiCache (Redis), and Managed Elasticsearch/OpenSearch.
Load Balancers/Ingress: Set up Application Load Balancers (ALBs) and Kubernetes Ingress controllers.
Secrets Management: Initial setup of AWS Secrets Manager to store configurations.
Agones on EKS: Initial Terraform to set up Agones on the EKS cluster for post-MVE game server hosting.
0.5.a. IaC Security Baseline Implementation (Enhanced & Validated - Principle R):
AWS Security Groups: Implement restrictive Security Groups. Default deny all traffic.
IAM Roles & Policies: Define minimal IAM roles for EKS nodes and Pods (using IRSA), adhering to the principle of least privilege.
Kubernetes NetworkPolicies: Implement default-deny NetworkPolicies within Kubernetes namespaces. Explicitly define allowed pod-to-pod communication paths and ports.
Pod Security Standards (PSS): Ensure all service deployments in Kubernetes adhere to baseline or restricted PSS profiles (run as non-root, read-only root filesystems, no privileged containers).
Logging & Monitoring Stubs: Enable basic audit logging from the EKS control plane to CloudWatch Logs.
Secure S3 Buckets: Enforce encryption at rest, block public access, use specific IAM policies.
Documentation & Validation:
Update /infra/README.md with setup instructions for the local Docker Compose environment.
Test docker-compose up -d locally and verify all stubs run and can communicate using their DNS service names.
Execute terraform apply for the cloud environment within a local Kubernetes cluster like minikube or k3d to validate the IaC scripts without provisioning cloud resources yet. Deploy smart stubs to this local cluster.
Validate security configurations on the local Kubernetes simulation (e.g., using kubectl describe networkpolicy).
Test that stubs correctly respond to enable_crypto flags in the Docker Compose environment.
Update docs/progress_logs/progress_phase_0.md:
Log Dockerfile creation for each smart stub, detailing base image choices and non-root user setup.
Detail the docker-compose.yml structure for the local simulation.
Explicitly document implemented IaC security measures and the methods used for their validation in the local Kubernetes test.
Note how stubs adhere to docs/SMART_STUBS_SPECIFICATION.md.
Design Rationale
A "Local-First, Cloud-Ready" approach provides the best of both worlds. Docker Compose offers a perfect, isolated environment for rapid local development, drastically reducing setup time. Developing Infrastructure-as-Code in parallel from Day One ensures that our architecture is not accidentally coupled to the local environment. It forces us to use cloud-native patterns like containerization, externalized configuration, and DNS-based service discovery, which guarantees a smooth, predictable migration to a production cloud environment when the time comes.
Operational Considerations
The docker-compose.yml file will be the standard development environment for the entire MVE lifecycle. The Terraform/Kubernetes manifests will be continuously tested and updated in CI as the project evolves, ensuring they are always in sync with the application architecture. For the future cloud deployment, these validated IaC scripts will be used to provision the staging and production environments on AWS/EKS.
Verification & Validation Criteria
Successful and secure deployment of all smart stubs to the local Docker Compose environment.
Successful validation of Terraform and Kubernetes IaC by deploying stubs to a local Kubernetes cluster (e.g., minikube).
Security configurations are verified via kubectl and peer review.
Stubs demonstrate correct behavioral changes based on the enable_crypto flag in the local environment.
Testing Methodologies
System testing of the full backend stack will be conducted using the Docker Compose environment. The cloud-ready IaC will be validated through deployment testing on a local Kubernetes cluster.
Version Control Strategy
Branching: All docker-compose.yml, Dockerfiles, Terraform scripts, and Kubernetes manifests are developed on feature/ branches and merged to develop.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security and DevOps Leads must review and approve all IaC before it is merged, paying special attention to the security baseline implementation (IAM roles, NetworkPolicies, etc.).
The use of non-root containers and minimal base images is a mandatory security checkpoint for all Dockerfiles.
ReviewedBy: DevOps Lead, SRE Lead, Security Lead, Lead Architect, Backend Tech Leads.
ReviewOutcome: IaC for Secure Smart Stubs (Dual-Mode Configurable, Reflecting Arbitrum Orbit L2 Simulation) Approved for Phase 0 Dev/Test.
ValidationMethod: Successful and secure deployment of all smart stubs to local Docker Compose. Security configurations for the cloud-ready IaC are verified by deploying to a local Kubernetes cluster and through peer review. Stubs demonstrate correct behavioral changes based on the enable_crypto flag.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 0.5: Functional IaC for 'Smart Stub' Distributed Rust Backend (Secure Baseline, Dual-Mode Configurable Stubs for Arbitrum Orbit Stack)." @Phase0/