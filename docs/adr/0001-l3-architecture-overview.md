# ADR-0001: L3 Architecture Overview

**Status:** Draft  
**Date:** 2024-09-08  
**Deciders:** Lead Architect, Blockchain Lead  

## Context

The Bunkerverse Platform requires a scalable blockchain infrastructure that can support both MVE (off-chain first) operations and future on-chain features. We need to decide on the Layer 3 architecture that will serve as the foundation for the BUNKERVERSE Netchain.

## Decision

We will implement a custom Layer 3 blockchain using Arbitrum Orbit technology, settling to Arbitrum One (L2), which in turn settles to Ethereum Mainnet (L1).

## Rationale

### Technical Benefits
- **Arbitrum Orbit** provides proven L3 framework with full EVM compatibility
- **Custom gas token** capability allows NTC token as native currency
- **Proven settlement path** L3→L2→L1 provides security guarantees
- **Developer tooling** compatible with existing Ethereum ecosystem

### Economic Model
- **NTC as gas token** aligns with tokenomics strategy
- **Reduced transaction costs** compared to L2 operations
- **Sovereignty** over chain parameters and governance

### Risk Mitigation
- **Battle-tested technology** reduces implementation risk
- **Fallback capability** to operate on L2 if needed
- **Gradual rollout** allows testing before full deployment

## Consequences

### Positive
- Full EVM compatibility enables existing Solidity contracts
- Reduced gas costs improve user experience
- Custom token economics possible with NTC gas token
- Strong security model inherited from Arbitrum/Ethereum

### Negative
- Additional complexity of three-layer architecture
- Dependency on Arbitrum One availability
- Cross-layer communication overhead
- Bridge security considerations

## Security Considerations & Threat Model Outline

### Threat Categories (STRIDE Analysis)

**Spoofing:**
- Multi-signature validation for critical operations
- Cryptographic proof verification at each layer
- Identity verification for validator nodes

**Tampering:**
- Immutable transaction logs across all layers
- Merkle proof verification for state transitions
- Cryptographic sealing of blocks

**Repudiation:**
- Complete audit trail from L1 to L3
- Timestamped transaction records
- Digital signatures for all operations

**Information Disclosure:**
- Public blockchain data by design
- Selective privacy through encryption where needed
- Secure key management for sensitive operations

**Denial of Service:**
- Rate limiting and gas mechanisms
- Multiple validator redundancy
- Fallback to lower layers during congestion

**Elevation of Privilege:**
- Role-based access control in smart contracts
- Multi-sig requirements for administrative functions
- Time-locked governance changes

### Security Controls
- **Bridge Security**: Multi-signature bridges with timelock mechanisms
- **Validator Set**: Decentralized validator network with slashing conditions
- **Governance**: Progressive decentralization with community oversight
- **Monitoring**: Real-time security monitoring and alerting

## Dual-Mode (Off-Chain/On-Chain) Design Considerations

### MVE Mode (Off-Chain First)
- **L3 Infrastructure**: Pre-deployed but not actively used for user transactions
- **Smart Contracts**: Deployed and tested but not integrated with user flows
- **NTC Token**: Contract exists but not used for gas or payments
- **Bridges**: Implemented but disabled for user transactions

### Full Blockchain Mode
- **Gradual Activation**: Phased rollout of on-chain features
- **User Migration**: Seamless transition from off-chain to on-chain operations
- **Data Migration**: Historical MVE data integration with blockchain state
- **Feature Parity**: All MVE features replicated on-chain

### Configuration Management
```rust
pub struct ChainConfig {
    pub enable_l3_transactions: bool,
    pub enable_nft_minting: bool,
    pub enable_token_transfers: bool,
    pub bridge_active: bool,
}
```

## Implementation Plan

### Phase 0 (Current)
- [x] Local development L3 setup
- [x] Basic smart contract deployment
- [x] NTC gas token configuration validation
- [ ] Security audit of contracts

### Phase 1 (MVE)
- [ ] Production L3 deployment
- [ ] Contract deployment to production
- [ ] Monitoring and alerting setup
- [ ] Bridge contracts (disabled state)

### Phase 2 (On-Chain Activation)
- [ ] Gradual feature activation
- [ ] User onboarding to on-chain features
- [ ] Full decentralization
- [ ] Governance token distribution

## Alternatives Considered

### Polygon CDK
- **Pros**: Mature ecosystem, good tooling
- **Cons**: Less flexibility for custom gas tokens, different architecture

### Optimism OP Stack
- **Pros**: Strong development team, growing ecosystem  
- **Cons**: Less mature than Arbitrum for L3 use cases

### Direct L2 Implementation
- **Pros**: Simpler architecture, lower costs
- **Cons**: Less sovereignty, no custom gas token, higher per-transaction costs

## References
- [Arbitrum Orbit Documentation](https://docs.arbitrum.io/launch-orbit-chain)
- [NTC Tokenomics Specification](../tokenomics/ntc-design.md)
- [Security Assessment Report](../security/l3-security-assessment.md)

---

**Review Required By:** Security Lead, Blockchain Lead, Lead Architect  
**Implementation Target:** Phase 0.2 (PoC), Phase 1.0 (Production)