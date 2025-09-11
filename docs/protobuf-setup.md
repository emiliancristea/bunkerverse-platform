# Protocol Buffer Setup & Validation

This document describes the Protocol Buffer (protobuf) toolchain setup, validation processes, and development workflows for the BUNKERVERSE platform.

## Overview

The platform uses Protocol Buffers v3 for:
- **API Contracts**: gRPC service definitions
- **Data Serialization**: Cross-language data exchange
- **Schema Validation**: Static analysis and breaking change detection
- **Code Generation**: Multi-language binding generation

## File Structure

```
schemas/proto/
├── enums.proto           # Core platform enumerations
├── types.proto           # Domain-specific types
├── events.proto          # Event sourcing definitions
└── services/
    ├── identity_service.proto    # Player identity & authentication
    ├── account_service.proto     # Account management
    ├── marketplace_service.proto # NFT marketplace operations
    └── indexer_service.proto     # Blockchain data indexing

buf.yaml                  # Buf CLI configuration
buf.gen.yaml             # Code generation configuration
buf.work.yaml            # Workspace configuration
```

## Configuration Files

### buf.yaml
Primary configuration for Protocol Buffer linting and validation:

- **Lint Rules**: BASIC, COMMENTS, FILE_LAYOUT, PACKAGE_DEFINED
- **Breaking Change Detection**: FILE-level tracking
- **Style Enforcement**: Service suffix requirements, enum naming
- **Dependencies**: googleapis, grpc-gateway

### buf.gen.yaml
Code generation configuration for multiple target languages:

- **Rust**: `libs/common-rust/src/generated` (with serde support)
- **TypeScript**: `services/web-client/src/generated` (ES6 modules)
- **C++**: `services/cxx-qt-client/src/generated` (Qt integration)
- **Go**: `tools/proto-gen/generated` (tooling support)

### buf.work.yaml
Workspace configuration for monorepo support:

- **Directories**: `schemas/proto`
- **Ignores**: Third-party proto files
- **Unified Linting**: Consistent rules across workspace

## Development Workflow

### 1. Schema Development

```bash
# Edit protobuf schemas in schemas/proto/
vim schemas/proto/types.proto

# Validate syntax and style
make proto-lint

# Generate code for all target languages  
make proto-gen

# Build and test generated code
make build test
```

### 2. Breaking Change Management

```bash
# Check for breaking changes against main branch
make proto-breaking

# If breaking changes are necessary:
# 1. Document in CHANGELOG.md
# 2. Update version numbers
# 3. Coordinate with dependent services
```

### 3. Code Generation Integration

Generated code is automatically integrated into:

- **Rust Services**: `common-rust` crate with `prost` serialization
- **Web Client**: TypeScript definitions for frontend
- **Qt Client**: C++ bindings for desktop application
- **Development Tools**: Go utilities for proto manipulation

## Validation Pipeline

### Static Analysis

1. **Syntax Validation**: buf config validate
2. **Lint Rules**: Comprehensive style checking
3. **Field Numbering**: Sequential validation
4. **Breaking Changes**: Wire format compatibility

### Security Validation

1. **Sensitive Data Detection**: Automatic scanning for credentials
2. **Field Access Patterns**: Validation of security-critical fields
3. **Enum Zero Values**: Ensure proper default handling
4. **Required Fields**: Validation of critical data requirements

### Generated Code Validation

1. **Compilation Checks**: All target languages compile successfully
2. **Type Safety**: Generated types match schema definitions
3. **Serialization Tests**: Round-trip data integrity
4. **Integration Tests**: Cross-service communication validation

## CI/CD Integration

### GitHub Actions Workflow

- **Trigger**: Changes to `schemas/proto/**` or buf configuration
- **Jobs**: 
  - proto-lint: Validate syntax and style
  - proto-generate: Generate and compile code
  - schema-validation: Security and consistency checks

### Local Development

```bash
# Install development tools
make install-tools

# Run full validation pipeline
make validate

# Quick development cycle  
make dev
```

## Best Practices

### Schema Design

1. **Backward Compatibility**: Never remove or renumber fields
2. **Field Naming**: Use snake_case consistently
3. **Enum Design**: Always include `_UNSPECIFIED = 0` default
4. **Service Naming**: Use `Service` suffix for gRPC services
5. **Package Structure**: Organize by domain boundaries

### Security Considerations

1. **No Secrets**: Never include credentials in proto definitions
2. **Field Validation**: Mark security-critical fields as required
3. **Access Control**: Document service-level authentication requirements
4. **Data Classification**: Mark sensitive fields in comments

### Performance Optimization

1. **Field Ordering**: Place frequently accessed fields first
2. **Message Size**: Keep messages under 4MB for optimal performance
3. **Repeated Fields**: Use appropriate collection types
4. **Optional Fields**: Minimize optional field usage for wire efficiency

## Troubleshooting

### Common Issues

1. **Compilation Errors**: Check generated code directories exist
2. **Breaking Changes**: Use buf breaking to identify specific changes
3. **Import Errors**: Verify buf.yaml dependencies are correct
4. **Version Conflicts**: Ensure buf CLI and plugin versions align

### Debug Commands

```bash
# Verbose linting output
buf lint --error-format=text

# Debug code generation
buf generate --debug

# Validate specific proto file
buf lint schemas/proto/types.proto

# Check breaking changes with detailed output
buf breaking --against main --error-format=json
```

## Tool Versions

- **buf CLI**: v1.28.1+
- **Protocol Buffers**: v3.21+
- **Rust prost**: v0.12+
- **TypeScript protobuf**: v5.0+

## References

- [Buf CLI Documentation](https://docs.buf.build/)
- [Protocol Buffers Language Guide](https://developers.google.com/protocol-buffers/docs/proto3)
- [gRPC Best Practices](https://grpc.io/docs/guides/performance/)
- [Protobuf Style Guide](https://developers.google.com/protocol-buffers/docs/style)