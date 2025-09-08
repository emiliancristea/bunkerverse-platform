Task 7.4: Intensive Monitoring & Observation of Staging Environment Under Diverse External User Load
(Principles K, P, R)
Technical Reference
Production Monitoring Stack (Prometheus, Grafana) Dashboards
Centralized Logging Platform (e.g., ELK stack or CloudWatch Logs Insights)
Client Crash Reporting Service (e.g., Sentry)
docs/INCIDENT_RESPONSE_PLAN_DRAFT.md
Context/Problem Statement
The closed external test (Task 7.3) provides the first opportunity to observe the MVE platform under a load that is not synthetic but is instead real, diverse, and unpredictable. While testers will report bugs they consciously find, a wealth of critical information about performance bottlenecks, unexpected usage patterns, hardware-specific issues, and silent failures can only be discovered through intensive, real-time monitoring of the entire system stack. This task is the active, operational counterpart to the testing itself.
Measurable Objectives
Continuous, real-time monitoring of all backend services, the L3 sequencer, and client-side crash reports is maintained throughout the external testing period.
Key monitoring observations, especially regarding performance bottlenecks and NAR stability on diverse hardware, are logged and analyzed.
A daily summary report of system health and performance anomalies is generated for the triage team.
Any P0-level incidents on the staging environment (e.g., service outages) are responded to according to the incident response plan.
Implementation Guidance
Action: Continuously and intensively monitor the production-like staging environment using all established tools (Prometheus, Grafana, Sentry). The focus is on analyzing how real, diverse, and unpredictable external user interactions and hardware profiles affect the Rust backend services and the client-side Rust NAR performance.
Live Monitoring & Analysis:
o Backend Performance (SRE/DevOps Team):
Monitor L3/Indexer Grafana dashboards for L3 sequencer TPS, latency, Indexer database performance, and resource utilization. Look for unexpected query patterns hitting the Indexer or transaction hotspots caused by real user behavior.
Monitor all Rust microservices for CPU/memory usage, gRPC error rates, and API latencies.
o Client-Side Performance & Errors (Client Team & AI Lead):
Actively analyze Sentry (or equivalent) for client-side crashes, especially those related to the Rust NAR FFI boundary or llama.cpp itself. Correlate crash reports with tester hardware specifications.
NAR Performance Analysis: Actively collect structured feedback and logs (where provided by testers) on NAR inference times, CPU/RAM impact, and any llama.cpp errors. This is the most critical data for understanding the real-world viability of the local AI approach across different hardware (Windows/Nvidia, Windows/AMD, macOS/ARM-Metal, etc.). Identify if specific hardware configurations are prone to poor performance or instability.
o Log Aggregation & Anomaly Detection:
Use centralized logging to query for error patterns or performance degradation across all services.
Look for unforeseen issues that only appear with diverse, concurrent user interactions (e.g., race conditions in the marketplace, unexpected mission progression conflicts).
o Staging Incident Response: Maintain readiness to address any staging environment issues (service outages, database problems) that block testers, following a streamlined version of the production incident response plan.
Update docs/progress_logs/progress_phase_7.md:
Log key monitoring observations, focusing on the impact of diverse external user patterns.
Document any significant performance variations or stability issues reported or observed for the Rust NAR on specific tester hardware configurations.
Detail any staging alerts that were triggered by external tester load and the responses taken.
Design Rationale
Active monitoring is a proactive approach to quality assurance. It allows the team to identify and diagnose problems that testers may not even be aware of, such as backend performance degradation or silent data corruption. For a novel feature like the local NAR, gathering real-world performance data across a wide hardware matrix is the only way to validate its feasibility and to identify necessary optimizations before a public launch.
Operational Considerations
Local-First Simulation: The entire "staging environment" is running in the local Docker Compose simulation on a single host machine. This makes monitoring both easier (all resources are centralized) and more critical (the entire system shares the resources of one machine). The performance of this host machine under load will be a key observation.
Cloud-Ready: The monitoring dashboards, alert configurations, and log query patterns established during this task are direct rehearsals for production operations. These same dashboards will be recreated for the production cloud environment, using the data gathered here as a baseline for setting alert thresholds.
Verification & Validation Criteria
Monitoring dashboards are analyzed daily, and reports are generated summarizing system behavior under real user load.
Tester feedback on performance (especially NAR) is collected, documented, and correlated with system metrics where possible.
Any staging environment incidents are successfully resolved in a timely manner.
Testing Methodologies
System Monitoring: Real-time observation of application and infrastructure metrics.
Log Analysis: Proactive querying of centralized logs for anomaly detection.
Performance Analysis: Correlating user-reported performance issues with backend and client-side metrics.
Version Control Strategy
Branching: N/A (Operational task). Any necessary changes to monitoring configurations (e.g., new Grafana dashboards) will be done in a feature/monitoring-updates branch.
Commits: The Git Commit message for this task will be exactly as specified.
Security Audit & Compliance Checkpoints
The Security Lead will have access to all monitoring and logging platforms to independently watch for security-related anomalies, such as high rates of authorization failures, anomalous API usage patterns, or potential DoS attacks against the staging environment.
ReviewedBy: SRE Lead, DevOps Lead, All Tech Leads, Lead Architect, AI Lead.
ReviewOutcome: Staging Performance & Stability Under Diverse External Load Assessed and Understood.
ValidationMethod: Monitoring dashboards are analyzed and reports are generated. Tester feedback on performance is collected, documented, and correlated with system metrics.
Git Commit here: @https://github.com/emiliancristea/bunkerverse-platform.git "Phase 7.4: Staging Environment Monitoring & Observations During Diverse Live External User Testing." @Phase7/