# Branch Protection Rules Configuration

This document outlines the required GitHub branch protection rules for the BunkerVerse Platform repository.

## Protected Branches

### `main` Branch
The main branch represents production-ready code and requires the strictest protection.

#### Required Settings:
- **Require pull request reviews before merging**
  - Required approving reviews: 2
  - Dismiss stale pull request approvals when new commits are pushed: ✓
  - Require review from CODEOWNERS: ✓
  - Restrict who can dismiss pull request reviews: Admins only

- **Require status checks to pass before merging**
  - Require branches to be up to date before merging: ✓
  - Required status checks:
    - `Netchain Backend CI / All Checks Complete`
    - `Netchain Client CI / All Client Checks Complete`
    - `Netchain Game Server Stubs CI / All Checks Complete`
    - `Protobuf Validation / validate`

- **Require conversation resolution before merging**: ✓

- **Require signed commits**: ✓

- **Include administrators**: ✗ (Admins can bypass in emergencies)

- **Restrict who can push to matching branches**
  - Users/teams with push access: Release managers only

- **Rules applied to everyone including administrators**: ✗

- **Allow force pushes**: ✗

- **Allow deletions**: ✗

### `develop` Branch
The develop branch is the integration branch for ongoing development.

#### Required Settings:
- **Require pull request reviews before merging**
  - Required approving reviews: 1
  - Dismiss stale pull request approvals when new commits are pushed: ✓
  - Require review from CODEOWNERS: ✗
  
- **Require status checks to pass before merging**
  - Require branches to be up to date before merging: ✓
  - Required status checks:
    - `Netchain Backend CI / All Checks Complete`
    - `Netchain Client CI / All Client Checks Complete`
    - `Netchain Game Server Stubs CI / All Checks Complete`
    - `Protobuf Validation / validate`

- **Require conversation resolution before merging**: ✓

- **Require signed commits**: ✗ (Recommended but not required)

- **Include administrators**: ✗

- **Restrict who can push to matching branches**: ✗ (All contributors can create PRs)

- **Allow force pushes**: ✗

- **Allow deletions**: ✗

## CI/CD Workflow Requirements

All pull requests must pass the following CI checks before merging:

### Backend Services
- Rust formatting check (`cargo fmt`)
- Clippy lints (with strict configuration)
- Build in release mode
- Unit and integration tests
- Security scans:
  - `cargo audit` for dependency vulnerabilities
  - `cargo deny` for license compliance
  - Trivy container scanning for Docker images

### Client Application
- Rust formatting and Clippy for app logic and NAR library
- C++ formatting check with clang-format
- QML linting with qmllint
- Build NAR library and client app logic
- Security scans for both Rust and C++ components
- Unit tests for all components

### Game Server Stubs
- C++ formatting and linting
- Build validation
- Static analysis with cppcheck

## Implementation Steps

1. **Navigate to Repository Settings**
   - Go to Settings → Branches in your GitHub repository

2. **Add Branch Protection Rule**
   - Click "Add rule"
   - Enter branch name pattern (e.g., `main` or `develop`)

3. **Configure Protection Settings**
   - Apply the settings listed above for each protected branch

4. **Save Changes**
   - Click "Create" or "Save changes"

## Verification

After configuring branch protection rules:

1. Create a test PR to verify all status checks are required
2. Confirm that merging is blocked until all checks pass
3. Verify that direct pushes to protected branches are blocked
4. Test that PR reviews are required as configured

## Exceptions and Emergency Procedures

In case of critical production issues requiring immediate fixes:

1. Repository admins can temporarily disable "Include administrators" to push emergency fixes
2. All emergency bypasses must be documented in the PR description
3. Regular protection rules must be re-enabled immediately after the emergency fix
4. A post-mortem must be conducted for any emergency bypass

## Review and Updates

Branch protection rules should be reviewed quarterly and updated as needed based on:
- Team size changes
- New CI/CD requirements
- Security policy updates
- Lessons learned from incidents

## Contact

For questions or changes to branch protection rules, contact:
- DevOps Lead
- Security Lead
- Repository Administrators