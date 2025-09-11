# BUNKERVERSE Smart Contract Evolution Plan
**Schema Changes & Upgradeability Strategy**

---

## Document Information
- **Document Type**: Technical Architecture Plan
- **Version**: v1.0
- **Date Created**: 2025-01-09
- **Last Updated**: 2025-01-09
- **Status**: Draft - Ready for Review
- **Scope**: L3 Smart Contract evolution, proxy patterns, schema migration

---

## Executive Summary

This document outlines the strategy for handling schema changes and smart contract evolution in the BUNKERVERSE Platform's L3 Arbitrum Orbit chain. The plan ensures backward compatibility, secure upgradeability, and seamless migration of on-chain data as the platform evolves.

### Key Strategies
- **Proxy Pattern Implementation** for upgradeable smart contracts
- **Versioned Event Schemas** with backward compatibility
- **Progressive Migration Strategy** for large-scale changes
- **Multi-Sig Governance** for upgrade authorization
- **Emergency Procedures** for critical security updates

---

## L3 Smart Contract Architecture

### Core Contract Structure
```
L3 Smart Contracts (Netchain)
├── Proxy Contracts (Immutable)
│   ├── UserRegistryProxy
│   ├── NftContractsProxy
│   ├── MarketplaceProxy
│   └── StakingProxy
├── Implementation Contracts (Upgradeable)
│   ├── UserRegistryV1 → UserRegistryV2 → ...
│   ├── NftContractsV1 → NftContractsV2 → ...
│   ├── MarketplaceV1 → MarketplaceV2 → ...
│   └── StakingV1 → StakingV2 → ...
└── Storage Contracts (Persistent)
    ├── UserStorage
    ├── NftStorage
    ├── MarketplaceStorage
    └── StakingStorage
```

### Proxy Pattern Implementation

#### 1. Transparent Proxy Pattern
```solidity
// Example: UserRegistryProxy.sol
contract UserRegistryProxy is TransparentUpgradeableProxy {
    constructor(
        address _logic,
        address _admin,
        bytes memory _data
    ) TransparentUpgradeableProxy(_logic, _admin, _data) {}
}

// UserRegistry Implementation V1
contract UserRegistryV1 {
    // Storage layout V1
    mapping(address => AgentChainState) public playerStates;
    uint256 public totalPlayers;
    
    // Events with schema version
    event UserRegistered(
        address indexed player,
        string playerId,
        uint256 schemaVersion  // Always include schema version
    );
}

// UserRegistry Implementation V2 (Future)
contract UserRegistryV2 {
    // CRITICAL: Maintain storage layout compatibility
    mapping(address => AgentChainState) public playerStates; // Slot 0
    uint256 public totalPlayers;                             // Slot 1
    
    // New storage variables MUST be appended
    mapping(address => PlayerPreferences) public playerPrefs; // Slot 2
    
    // Events with updated schema
    event UserRegisteredV2(
        address indexed player,
        string playerId,
        PlayerPreferences preferences,
        uint256 schemaVersion  // Incremented schema version
    );
}
```

#### 2. Upgrade Authorization
```solidity
// Multi-sig controlled upgrade mechanism
contract UpgradeController {
    address[] public signers;
    uint256 public requiredSignatures;
    uint256 public upgradeDelay = 48 hours; // Time lock for upgrades
    
    struct UpgradeProposal {
        address proxyContract;
        address newImplementation;
        uint256 proposedAt;
        uint256 approvalsCount;
        mapping(address => bool) approvals;
        bool executed;
    }
    
    mapping(bytes32 => UpgradeProposal) public proposals;
    
    function proposeUpgrade(
        address _proxy,
        address _newImplementation
    ) external onlyMultiSig {
        // Create upgrade proposal with time lock
    }
    
    function approveUpgrade(bytes32 _proposalId) external onlyMultiSig {
        // Approve upgrade proposal
    }
    
    function executeUpgrade(bytes32 _proposalId) external {
        UpgradeProposal storage proposal = proposals[_proposalId];
        require(proposal.approvalsCount >= requiredSignatures, "Insufficient approvals");
        require(block.timestamp >= proposal.proposedAt + upgradeDelay, "Time lock active");
        
        // Execute the upgrade
        ITransparentUpgradeableProxy(proposal.proxyContract).upgradeTo(
            proposal.newImplementation
        );
    }
}
```

---

## Event Schema Evolution Strategy

### Schema Versioning
All events emitted by L3 contracts include a `schemaVersion` field for tracking:

```protobuf
message CanonicalEventProto {
  string event_id = 1;
  uint64 block_number = 2;
  // ... other fields ...
  uint32 schema_version = 20;  // Schema version tracking
}

// Example event evolution:
message UserRegisteredPayloadProto {
  string player_id = 1;
  string l3_wallet_address = 2;
  string bunker_tag = 3;
  int64 registration_timestamp = 4;
  string registration_tx_hash = 5;
  uint32 schema_version = 6;  // V1 = 1, V2 = 2, etc.
  
  // V2 additions (optional for backward compatibility):
  optional PlayerPreferencesProto preferences = 7;
  optional ReferralDataProto referral_info = 8;
}
```

### Backward Compatibility Rules

#### 1. Additive Changes (Safe)
```solidity
// V1 Event
event NftMinted(
    address indexed owner,
    uint256 indexed tokenId,
    uint256 schemaVersion
);

// V2 Event (Backward Compatible)
event NftMintedV2(
    address indexed owner,
    uint256 indexed tokenId,
    string metadataUri,      // New field
    uint256 mintPrice,       // New field
    uint256 schemaVersion    // Incremented version
);

// Both events can coexist, indexer handles both
```

#### 2. Breaking Changes (Requires Migration)
```solidity
// V1: Single address field
event PlayerRegistered(
    address indexed playerAddress,
    string playerId,
    uint256 schemaVersion
);

// V2: Multiple addresses (Breaking change)
event PlayerRegisteredV2(
    address indexed primaryAddress,
    address[] allAddresses,    // Breaking: changes data structure
    string playerId,
    uint256 schemaVersion
);

// Migration required for indexer compatibility
```

---

## Migration Strategies

### 1. Rolling Migration (Preferred)
For non-breaking schema changes:

```typescript
// Indexer migration strategy
class EventProcessor {
  processUserRegisteredEvent(event: any) {
    if (event.schemaVersion === 1) {
      return this.processV1Format(event);
    } else if (event.schemaVersion === 2) {
      return this.processV2Format(event);
    }
    throw new Error(`Unsupported schema version: ${event.schemaVersion}`);
  }
  
  processV1Format(event: any) {
    // Handle V1 format
    return {
      playerId: event.playerId,
      address: event.playerAddress,
      preferences: null, // Default for missing V2 fields
      registrationTime: event.timestamp
    };
  }
  
  processV2Format(event: any) {
    // Handle V2 format
    return {
      playerId: event.playerId,
      address: event.primaryAddress,
      preferences: event.preferences,
      registrationTime: event.timestamp
    };
  }
}
```

### 2. Blue-Green Migration (For Breaking Changes)
```typescript
// Blue-Green deployment for indexer
class MigrationManager {
  async executeBlueGreenMigration() {
    // Step 1: Deploy new indexer version (Green)
    const greenIndexer = await this.deployGreenIndexer();
    
    // Step 2: Historical re-indexing in background
    await greenIndexer.reindexFromBlock(0);
    
    // Step 3: Sync remaining blocks
    await greenIndexer.catchUpToLatest();
    
    // Step 4: Switch traffic to green
    await this.switchToGreen();
    
    // Step 5: Decommission blue indexer
    await this.decommissionBlue();
  }
}
```

### 3. Progressive Migration
For large datasets:

```solidity
contract MigrationHelper {
    uint256 public migrationProgress;
    uint256 public totalRecords;
    bool public migrationComplete;
    
    function migrateBatch(uint256 batchSize) external onlyAdmin {
        require(!migrationComplete, "Migration already complete");
        
        uint256 endIndex = min(migrationProgress + batchSize, totalRecords);
        
        for (uint256 i = migrationProgress; i < endIndex; i++) {
            migrateRecord(i);
        }
        
        migrationProgress = endIndex;
        
        if (migrationProgress >= totalRecords) {
            migrationComplete = true;
            emit MigrationCompleted(totalRecords);
        }
    }
    
    function migrateRecord(uint256 index) internal {
        // Migrate individual record from old format to new format
    }
}
```

---

## Indexer Evolution Strategy

### Multi-Version Event Processing
```typescript
interface EventHandler {
  supportedVersions: number[];
  process(event: BlockchainEvent): ProcessedEvent;
}

class NftMintedHandler implements EventHandler {
  supportedVersions = [1, 2, 3];
  
  process(event: BlockchainEvent): ProcessedEvent {
    switch (event.schemaVersion) {
      case 1:
        return this.processV1(event);
      case 2:
        return this.processV2(event);
      case 3:
        return this.processV3(event);
      default:
        throw new UnsupportedVersionError(event.schemaVersion);
    }
  }
  
  private processV1(event: any): ProcessedEvent {
    return {
      nftId: event.tokenId.toString(),
      owner: event.owner,
      metadata: null, // V1 didn't have metadata
      price: null,    // V1 didn't track mint price
      version: 1
    };
  }
  
  private processV2(event: any): ProcessedEvent {
    return {
      nftId: event.tokenId.toString(),
      owner: event.owner,
      metadata: event.metadataUri,
      price: event.mintPrice,
      version: 2
    };
  }
}
```

### Schema Registry
```typescript
class SchemaRegistry {
  private schemas: Map<string, Map<number, EventSchema>> = new Map();
  
  registerSchema(eventType: string, version: number, schema: EventSchema) {
    if (!this.schemas.has(eventType)) {
      this.schemas.set(eventType, new Map());
    }
    this.schemas.get(eventType)!.set(version, schema);
  }
  
  getSchema(eventType: string, version: number): EventSchema | null {
    return this.schemas.get(eventType)?.get(version) ?? null;
  }
  
  validateEvent(event: BlockchainEvent): boolean {
    const schema = this.getSchema(event.type, event.schemaVersion);
    return schema ? schema.validate(event) : false;
  }
}
```

---

## Governance & Security

### Multi-Signature Upgrade Process
```typescript
interface UpgradeGovernance {
  // Required signatures for different types of upgrades
  ROUTINE_UPGRADE_THRESHOLD: 3; // out of 5 multisig
  EMERGENCY_UPGRADE_THRESHOLD: 2; // out of 5 multisig
  BREAKING_CHANGE_THRESHOLD: 4;   // out of 5 multisig
  
  // Time delays for different upgrade types
  ROUTINE_DELAY: 48 * 3600;     // 48 hours
  BREAKING_CHANGE_DELAY: 168 * 3600; // 7 days
  EMERGENCY_DELAY: 0;            // Immediate (security fixes)
}

class UpgradeProposal {
  proposalId: string;
  contractAddress: string;
  newImplementation: string;
  upgradeType: 'ROUTINE' | 'BREAKING' | 'EMERGENCY';
  proposedAt: number;
  approvals: string[];
  executedAt?: number;
  
  canExecute(): boolean {
    const requiredApprovals = this.getRequiredApprovals();
    const requiredDelay = this.getRequiredDelay();
    
    return (
      this.approvals.length >= requiredApprovals &&
      Date.now() >= this.proposedAt + requiredDelay
    );
  }
}
```

### Emergency Procedures
```solidity
contract EmergencyController {
    address[] public emergencyResponders;
    bool public emergencyPaused;
    
    modifier onlyEmergency() {
        bool isResponder = false;
        for (uint i = 0; i < emergencyResponders.length; i++) {
            if (msg.sender == emergencyResponders[i]) {
                isResponder = true;
                break;
            }
        }
        require(isResponder, "Only emergency responders");
        _;
    }
    
    function emergencyPause() external onlyEmergency {
        emergencyPaused = true;
        emit EmergencyPaused(msg.sender);
        
        // Pause all upgradeable contracts
        pauseAllContracts();
    }
    
    function emergencyUpgrade(
        address proxy,
        address newImplementation
    ) external onlyEmergency {
        require(emergencyPaused, "System not in emergency mode");
        
        // Skip time delays for emergency upgrades
        ITransparentUpgradeableProxy(proxy).upgradeTo(newImplementation);
        
        emit EmergencyUpgrade(proxy, newImplementation);
    }
}
```

---

## Testing Strategy

### Upgrade Testing Pipeline
```typescript
class UpgradeTestSuite {
  async testUpgrade(
    currentVersion: string,
    newVersion: string
  ): Promise<TestResult> {
    
    // 1. Deploy new implementation to testnet
    const newImplementation = await this.deployImplementation(newVersion);
    
    // 2. Test storage compatibility
    const storageTest = await this.testStorageCompatibility(
      currentVersion,
      newVersion
    );
    
    // 3. Test event schema compatibility
    const eventTest = await this.testEventCompatibility(
      currentVersion,
      newVersion
    );
    
    // 4. Test indexer compatibility
    const indexerTest = await this.testIndexerCompatibility(
      newVersion
    );
    
    // 5. Performance regression tests
    const perfTest = await this.testPerformance(newVersion);
    
    return {
      storageCompatible: storageTest.passed,
      eventCompatible: eventTest.passed,
      indexerCompatible: indexerTest.passed,
      performanceRegression: perfTest.regression,
      overallResult: this.calculateOverallResult([
        storageTest, eventTest, indexerTest, perfTest
      ])
    };
  }
}
```

### Integration Testing
```typescript
class IntegrationTestRunner {
  async runFullStackTest(upgradeVersion: string) {
    // 1. Deploy contracts with upgrade
    const contracts = await this.deployUpgradedContracts(upgradeVersion);
    
    // 2. Deploy updated indexer
    const indexer = await this.deployUpdatedIndexer(upgradeVersion);
    
    // 3. Deploy updated backend services
    const services = await this.deployUpdatedServices(upgradeVersion);
    
    // 4. Run end-to-end tests
    const e2eResults = await this.runE2ETests({
      contracts,
      indexer,
      services
    });
    
    // 5. Validate data consistency
    const consistencyResults = await this.validateDataConsistency();
    
    return {
      e2eTests: e2eResults,
      dataConsistency: consistencyResults,
      success: e2eResults.passed && consistencyResults.passed
    };
  }
}
```

---

## Monitoring & Rollback Procedures

### Upgrade Monitoring
```typescript
class UpgradeMonitor {
  private metrics: MetricsCollector;
  private alerts: AlertManager;
  
  async monitorUpgrade(upgradeId: string) {
    const startTime = Date.now();
    const baselineMetrics = await this.collectBaselineMetrics();
    
    // Monitor key metrics post-upgrade
    const monitoringTasks = [
      this.monitorContractGasUsage(),
      this.monitorIndexerPerformance(),
      this.monitorErrorRates(),
      this.monitorDataIntegrity()
    ];
    
    const results = await Promise.all(monitoringTasks);
    
    if (this.detectIssues(results, baselineMetrics)) {
      await this.triggerRollbackAlert(upgradeId);
    }
  }
  
  private detectIssues(
    currentMetrics: Metrics[],
    baseline: Metrics
  ): boolean {
    return (
      currentMetrics.some(m => m.errorRate > baseline.errorRate * 1.5) ||
      currentMetrics.some(m => m.gasUsage > baseline.gasUsage * 1.2) ||
      currentMetrics.some(m => m.processingTime > baseline.processingTime * 1.3)
    );
  }
}
```

### Rollback Procedures
```typescript
class RollbackManager {
  async executeRollback(upgradeId: string): Promise<RollbackResult> {
    console.log(`Initiating rollback for upgrade ${upgradeId}`);
    
    try {
      // 1. Pause new transactions
      await this.pauseIncomingTransactions();
      
      // 2. Rollback smart contracts to previous version
      await this.rollbackContracts(upgradeId);
      
      // 3. Rollback indexer to previous version
      await this.rollbackIndexer(upgradeId);
      
      // 4. Rollback backend services
      await this.rollbackServices(upgradeId);
      
      // 5. Verify system health
      const healthCheck = await this.performHealthCheck();
      
      if (healthCheck.passed) {
        // 6. Resume operations
        await this.resumeOperations();
        return { success: true, message: "Rollback completed successfully" };
      } else {
        throw new Error("Health check failed after rollback");
      }
      
    } catch (error) {
      console.error("Rollback failed:", error);
      await this.alertEmergencyTeam(error);
      return { success: false, message: error.message };
    }
  }
}
```

---

## Documentation & Communication

### Change Documentation Template
```markdown
# Smart Contract Upgrade: [Contract Name] v[X.Y] → v[X.Z]

## Summary
Brief description of the changes and why they were needed.

## Changes
### Breaking Changes
- List any breaking changes
- Impact on existing functionality
- Required migration steps

### New Features
- New functionality added
- New events emitted
- New storage variables

### Bug Fixes
- Bugs fixed in this upgrade
- Security issues addressed

## Migration Guide
### For Developers
- API changes
- New integration requirements
- Deprecated functionality

### For Users
- User-facing changes
- Required actions (if any)

## Testing
- Test results summary
- Performance impact analysis
- Security audit results

## Deployment Schedule
- Testnet deployment: [Date/Time]
- Mainnet deployment: [Date/Time]
- Monitoring period: [Duration]

## Rollback Plan
- Rollback triggers
- Rollback procedure
- Emergency contacts
```

### Communication Strategy
```typescript
class CommunicationManager {
  async announceUpgrade(upgrade: UpgradeProposal) {
    // 1. Technical announcement
    await this.notifyDevelopers({
      audience: 'developers',
      channel: 'discord-dev-channel',
      content: this.generateTechnicalAnnouncement(upgrade)
    });
    
    // 2. User announcement
    await this.notifyUsers({
      audience: 'users',
      channel: 'discord-announcements',
      content: this.generateUserAnnouncement(upgrade)
    });
    
    // 3. Documentation update
    await this.updateDocumentation(upgrade);
    
    // 4. API documentation update
    await this.updateAPIDocumentation(upgrade);
  }
}
```

---

## Conclusion

This Smart Contract Evolution Plan provides a comprehensive strategy for managing schema changes and contract upgrades in the BUNKERVERSE Platform. The combination of proxy patterns, versioned events, and robust governance ensures that the platform can evolve while maintaining security, compatibility, and user trust.

### Key Success Factors
1. **Rigorous Testing**: All upgrades must pass comprehensive test suites
2. **Gradual Rollouts**: Use blue-green and progressive migration strategies
3. **Monitoring**: Continuous monitoring with automatic rollback triggers
4. **Governance**: Multi-signature approval with appropriate time delays
5. **Communication**: Clear communication to all stakeholders

### Next Steps
1. Implement proxy contracts for all L3 smart contracts
2. Establish multi-signature governance structure
3. Build upgrade testing pipeline
4. Implement monitoring and alerting systems
5. Create detailed operational procedures

---

**Document Maintained By**: BUNKERVERSE Architecture Team  
**Review Schedule**: Quarterly or before major upgrades  
**Contact**: Lead Architect for questions and updates