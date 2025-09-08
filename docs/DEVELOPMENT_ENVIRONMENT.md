# Development Environment Setup

This document provides detailed setup instructions for the Bunkerverse Platform development environment on Windows 11.

## Prerequisites

### Rust Toolchain
- **Version**: 1.80.0 (pinned via rust-toolchain.toml)
- **Installation**: 
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup toolchain install 1.80.0
  rustup default 1.80.0
  ```

### C++ Compiler
- **MSVC 2022**: Latest version from Visual Studio 2022
- **Build Tools**: Visual Studio Build Tools 2022 minimum
- **Required Components**: 
  - MSVC v143 compiler toolset
  - Windows 11 SDK (latest)

### Qt Framework
- **Version**: 6.9.2 (EXACT VERSION REQUIRED)
- **Installation Path**: `D:\DOCUMENTS\Qt\6.9.2\msvc2022_64`
- **Components Required**:
  - Qt6Core, Qt6Gui, Qt6Widgets, Qt6Qml, Qt6Quick, Qt6QuickControls2, Qt6Graphics
- **Installation**: Download from Qt official installer

### CXX-Qt Integration
- **Version**: 0.7.2 (pinned in Cargo.toml files)
- **Setup**: Configured automatically via Cargo dependencies
- **Compatibility**: Tested with Qt 6.9.2 and Rust 1.80.0

### CMake
- **Version**: 3.29.3 minimum
- **Installation**: Download from CMake official website
- **Usage**: For C++ builds and Qt integration

### Protocol Buffers
- **protoc**: Latest stable version
- **Installation**: 
  ```bash
  # Windows
  choco install protoc
  # Or download from GitHub releases
  ```

### Docker Desktop
- **Version**: Latest stable for Windows
- **Configuration**: WSL2 backend enabled
- **Usage**: Local backend simulation and testing

### Node.js (Optional)
- **Version**: As specified in .nvmrc (if present)
- **Usage**: Build tooling and development scripts

### Unreal Engine 5
- **Version**: Latest stable (for post-MVE development)
- **Installation**: Epic Games Launcher
- **Usage**: Game server development (Phase 1+)

## Security Tools

### Static Analysis (SAST)
- **Rust**: clippy with pedantic warnings enabled
- **C++**: MSVC static analyzer, cppcheck
- **Integration**: Pre-commit hooks enforce checks

### Dependency Scanning
- **cargo audit**: Rust vulnerability scanning
- **Installation**: 
  ```bash
  cargo install cargo-audit
  ```

### IDE Plugins
- **SonarLint**: For VS Code/Visual Studio
- **Rust Analyzer**: With clippy integration
- **Configuration**: Security-focused settings enabled

## Repository Setup

### Clone and Build
1. **Clone repository**:
   ```bash
   git clone https://github.com/emiliancristea/bunkerverse-platform.git
   cd bunkerverse-platform
   ```

2. **Verify toolchain**:
   ```bash
   rustc --version  # Should show 1.80.0
   cargo --version
   ```

3. **Build all components**:
   ```bash
   cargo build --all
   # C++ components (when available):
   # cmake -B build -S .
   # cmake --build build
   ```

4. **Run tests**:
   ```bash
   cargo test --all
   ```

### Pre-commit Setup
Pre-commit hooks are automatically configured to run:
- `rustfmt` for code formatting
- `clippy -- -D warnings -W clippy::pedantic` for strict linting
- `cargo audit --deny warnings` for security checks

### Git Configuration
```bash
git config user.name "Your Name"
git config user.email "your.email@company.com"
```

## Development Workflow

### Branching Strategy
- **main**: Production-ready code
- **develop**: Integration branch
- **feature/**: Feature branches off develop
- **hotfix/**: Emergency fixes off main

### Pull Request Process
1. Create feature branch from develop
2. Implement changes following coding standards
3. Ensure all pre-commit checks pass
4. Create PR with mandatory sections:
   - Description of changes
   - Security implications assessment
   - Testing performed
   - First Principles adherence verification

### Local Development Environment
- **Backend Services**: Run via Docker Compose (when available)
- **Client Application**: Build and run locally
- **Hot Reload**: Configured for rapid development

## Troubleshooting

### Common Issues

**Qt Path Issues**:
- Ensure Qt is installed at exact path: `D:\DOCUMENTS\Qt\6.9.2\msvc2022_64`
- Verify Qt environment variables are set

**Rust Toolchain Issues**:
- Use `rustup show` to verify active toolchain
- Reinstall if rust-toolchain.toml is not respected

**Build Failures**:
- Clear cargo cache: `cargo clean`
- Rebuild dependencies: `cargo update`
- Check MSVC installation

**Git Hook Failures**:
- Run `cargo fmt` manually before commit
- Fix clippy warnings: `cargo clippy --fix`
- Update dependencies: `cargo audit fix`

### Security Considerations
- **Never commit secrets** or configuration files with sensitive data
- **Use .gitignore** patterns for temporary and build files
- **Enable branch protection** on main/develop branches
- **Review security implications** of all dependency updates

## Validation
A successful environment setup should allow you to:
1. Clone the repository without errors
2. Build all Rust components
3. Run all tests successfully
4. Commit changes with pre-commit hooks passing
5. Create and build a simple "Hello World" application

---

**Last Updated**: Phase 0.1  
**Review Required**: When adding new tools or changing versions  
**Contact**: Development Team Lead for setup issues