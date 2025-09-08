# Control Center Client

Multi-language client application for the BUNKERVERSE platform.

## Architecture

- `rust-app-logic/` - Core application logic in Rust
- `qml-ui/` - User interface built with Qt/QML
- `cpp-shell/` - Qt C++ application shell integrating components
- `assets/` - UI assets (images, fonts, etc.)

## Technology Stack

- **Rust**: Business logic, API communication, data processing
- **Qt/QML**: Modern, declarative user interface  
- **CXX-Qt**: Safe Rust-C++ interop for Qt integration
- **C++17**: Qt application shell and native integrations

## Building

Requires Qt 6.9.2 SDK and CXX-Qt 0.7.2:

1. Install Qt SDK to `D:\DOCUMENTS\Qt\6.9.2\msvc2022_64`
2. Build Rust components: `cargo build -p client-app-logic`
3. Build C++ shell: `cmake --build build/`

See `../docs/DEVELOPMENT_ENVIRONMENT.md` for complete setup instructions.