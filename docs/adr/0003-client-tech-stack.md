# ADR-0003: Control Center Client Technology Stack

**Status:** Draft  
**Date:** 2024-09-08  
**Deciders:** Lead Architect, Frontend Lead  

## Context

The Bunkerverse Platform requires a cross-platform desktop client application that can integrate complex Rust business logic with a modern, responsive user interface. The client must support both MVE features and future blockchain integrations.

## Decision

We will use a **Rust + CXX-Qt + QML** architecture:
- **Rust**: Core application logic and business rules
- **CXX-Qt**: Safe FFI bridge between Rust and Qt
- **QML**: User interface and presentation layer
- **Qt 6.9.2**: UI framework and native integration

## Rationale

### Technical Benefits
- **Memory Safety**: Rust ensures memory safety for complex business logic
- **Performance**: Native performance with minimal FFI overhead (<0.01μs per call)
- **Cross-Platform**: Qt provides native look-and-feel across Windows, macOS, Linux
- **Modern UI**: QML enables modern, responsive user interfaces

### Integration Advantages
- **Type Safety**: CXX-Qt provides compile-time type safety across FFI boundary
- **Shared Logic**: Business logic in Rust can be shared with backend services
- **Native Integration**: Qt provides native OS integration (notifications, system tray, etc.)
- **Rapid Development**: QML enables rapid UI iteration and prototyping

### Ecosystem Fit
- **Rust Ecosystem**: Leverages existing Rust libraries and tooling
- **Qt Ecosystem**: Access to mature UI components and tools
- **Development Tools**: Excellent debugging and profiling capabilities
- **Community Support**: Strong communities for both Rust and Qt

## Consequences

### Positive
- Memory-safe application logic eliminates entire classes of bugs
- High-performance native application with minimal resource usage
- Cross-platform deployment reduces development and maintenance overhead
- Modern, responsive UI capabilities with QML
- Type-safe integration between business logic and UI

### Negative
- Complex build system requiring both Rust and Qt toolchains
- Learning curve for developers unfamiliar with QML or CXX-Qt
- Larger application size due to Qt runtime requirements
- Platform-specific Qt deployment considerations

## Security Considerations & Threat Model Outline

### Threat Categories (STRIDE Analysis)

**Spoofing:**
- Application signing and certificate validation
- User authentication before sensitive operations
- Process integrity validation

**Tampering:**
- Code signing verification on startup
- Runtime integrity checks for critical components
- Secure update mechanism with signature validation

**Repudiation:**
- User action logging with timestamps
- Audit trail for sensitive operations
- Digital signatures for critical transactions

**Information Disclosure:**
- Memory protection for sensitive data (credentials, private keys)
- Secure storage for user preferences and cached data
- Screen capture protection for sensitive screens

**Denial of Service:**
- Input validation to prevent UI lockups
- Resource limits for background operations
- Graceful handling of network failures

**Elevation of Privilege:**
- Principle of least privilege for system access
- Sandbox mode for untrusted content
- Secure handling of administrative functions

### Security Controls
- **Memory Safety**: Rust ownership system prevents memory corruption
- **FFI Security**: CXX-Qt type system prevents common FFI vulnerabilities
- **Input Validation**: All user inputs validated at FFI boundary
- **Secure Storage**: Encrypted storage for sensitive configuration
- **Update Security**: Signed updates with rollback capability

## Dual-Mode (Off-Chain/On-Chain) Design Considerations

### MVE Mode (Off-Chain First)
- **UI Features**: Full marketplace, missions, forum, and social features
- **Hidden Elements**: Blockchain-related UI components hidden but present
- **Configuration**: `show_crypto` flag controls UI visibility
- **Data Flow**: All data flows through Rust business logic layer

### Full Blockchain Mode
- **Progressive Disclosure**: Blockchain features revealed based on user preferences
- **Wallet Integration**: Native wallet connection and transaction signing
- **Real-time Updates**: Blockchain event integration with UI updates
- **Cross-Platform**: Consistent blockchain features across all platforms

### Configuration Management
```rust
pub struct ClientConfig {
    pub show_crypto: bool,
    pub enable_wallet_connection: bool,
    pub blockchain_network: String,
    pub ui_theme: UITheme,
}

// QML integration
Q_OBJECT_RUST! {
    pub struct AppConfig {
        #[qproperty(bool, show_crypto)]
        pub show_crypto: bool,
        
        #[qproperty(bool, wallet_enabled)]  
        pub wallet_enabled: bool,
    }
}
```

## Architecture Details

### Component Architecture
```
┌─────────────────┐
│   QML UI Layer  │ ← User Interface
├─────────────────┤
│   CXX-Qt Bridge │ ← Type-safe FFI
├─────────────────┤  
│ Rust App Logic  │ ← Business Logic
├─────────────────┤
│  Native APIs    │ ← System Integration
└─────────────────┘
```

### Data Flow
1. **User Interaction**: QML captures user input
2. **FFI Bridge**: CXX-Qt marshals data to Rust
3. **Business Logic**: Rust processes requests, calls backend APIs
4. **Response**: Results marshaled back to QML for display
5. **UI Update**: QML updates interface based on state changes

## Implementation Plan

### Phase 0 (Current)
- [x] PoC with basic Rust ↔ QML integration
- [x] FFI performance validation (<0.01μs overhead)
- [x] Memory safety verification with fuzz testing
- [ ] Security review of FFI boundaries

### Phase 1 (MVE)
- [ ] Complete UI implementation for all MVE features
- [ ] Cross-platform build and deployment
- [ ] User acceptance testing
- [ ] Performance optimization and profiling

### Phase 2 (On-Chain Integration)
- [ ] Wallet integration components
- [ ] Blockchain transaction UI flows
- [ ] Real-time blockchain event handling
- [ ] Advanced crypto features UI

## Alternatives Considered

### Tauri (Rust + Web Technologies)
- **Pros**: Familiar web technologies, smaller bundle size
- **Cons**: Less native integration, potential security issues with web technologies

### Flutter
- **Pros**: Cross-platform, good performance, modern UI
- **Cons**: Dart language requirement, less native integration

### Electron
- **Pros**: Familiar web technologies, rapid development
- **Cons**: High memory usage, security concerns, performance issues

### Native Qt C++
- **Pros**: Maximum performance, full Qt ecosystem
- **Cons**: Memory safety issues, complex business logic sharing

### Pure Rust (egui/iced)
- **Pros**: Memory safety, single language
- **Cons**: Less mature UI ecosystem, platform integration challenges

## Performance Benchmarks (PoC Results)
- **FFI Call Overhead**: <0.01μs average (excellent)
- **Memory Usage**: ~50MB base (reasonable for Qt application)
- **Startup Time**: <2 seconds cold start
- **UI Responsiveness**: 60 FPS consistently maintained
- **Cross-Platform**: Verified on Windows 11, planned for macOS/Linux

## Technical Requirements

### Build Dependencies
- **Rust**: 1.80.0 (pinned via rust-toolchain.toml)
- **Qt**: 6.9.2 MSVC 2022 64-bit (exact version required)
- **CXX-Qt**: 0.7.2 (pinned for Qt compatibility)
- **CMake**: 3.29.3+ for build orchestration

### Deployment
- **Windows**: MSVC build with Qt runtime
- **Code Signing**: Required for Windows deployment
- **Update Mechanism**: Secure automatic updates
- **Installer**: Professional installer package

## References
- [CXX-Qt Documentation](https://kdab.github.io/cxx-qt/)
- [Qt 6.9.2 Documentation](https://doc.qt.io/)
- [Performance Benchmarks](../performance/client-benchmarks.md)
- [Security Assessment](../security/client-security-review.md)

---

**Review Required By:** Security Lead, Frontend Lead, UX Lead  
**Implementation Target:** Phase 0.2 (PoC Complete), Phase 1.0 (Production)