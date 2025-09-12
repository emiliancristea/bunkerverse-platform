# Code Signing Setup Guide

This document describes how to set up code signing for BunkerVerse Control Center installers across all platforms.

## Overview

Code signing ensures that our software hasn't been tampered with and comes from a trusted source (BunkerVerse Corporation). Each platform has its own signing requirements and process.

## Platform Requirements

### Windows Code Signing

#### Certificate Types
- **Standard Code Signing Certificate**: Basic signing, shows publisher name
- **EV (Extended Validation) Certificate**: Immediate SmartScreen reputation, hardware token required

#### Required Secrets in GitHub
```yaml
WINDOWS_CERT_BASE64: Base64-encoded .pfx certificate file
WINDOWS_CERT_PASSWORD: Password for the certificate
```

#### Converting Certificate to Base64
```powershell
# PowerShell
$cert = Get-Content "certificate.pfx" -Encoding Byte
[System.Convert]::ToBase64String($cert) | Out-File "cert-base64.txt"
```

#### Local Testing
```powershell
# Sign an MSI locally
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com /fd sha256 /d "BunkerVerse Control Center" installer.msi

# Verify signature
signtool verify /pa /v installer.msi
```

### macOS Code Signing

#### Requirements
- Apple Developer Program membership ($99/year)
- Developer ID Application certificate
- Developer ID Installer certificate (for .pkg)
- Notarization credentials

#### Required Secrets in GitHub
```yaml
MACOS_CERT_BASE64: Base64-encoded .p12 certificate
MACOS_CERT_PASSWORD: Certificate password
MACOS_KEYCHAIN_PASSWORD: Temporary keychain password
APPLE_TEAM_ID: Your Apple Team ID (e.g., "ABC123DEF4")
APPLE_NOTARIZATION_EMAIL: Apple ID email for notarization
APPLE_NOTARIZATION_PASSWORD: App-specific password for notarization
```

#### Creating App-Specific Password
1. Sign in to https://appleid.apple.com
2. Go to Security → App-Specific Passwords
3. Generate a new password for "BunkerVerse CI"
4. Save this password as `APPLE_NOTARIZATION_PASSWORD`

#### Converting Certificate to Base64
```bash
# Convert .p12 to base64
base64 -i certificate.p12 -o cert-base64.txt
```

#### Local Testing
```bash
# Sign an app bundle
codesign --force --verify --verbose --sign "Developer ID Application: Your Name (TEAMID)" \
  --options runtime --timestamp "BunkerVerse Control Center.app"

# Create and sign DMG
create-dmg ... output.dmg
codesign --force --sign "Developer ID Application: Your Name (TEAMID)" output.dmg

# Notarize
xcrun notarytool submit output.dmg --apple-id email@example.com \
  --password app-specific-password --team-id TEAMID --wait

# Staple ticket
xcrun stapler staple output.dmg

# Verify
spctl -a -vvv -t install output.dmg
```

### Linux Code Signing (Optional)

Linux doesn't require code signing, but we can provide GPG signatures for verification.

#### Required Secrets in GitHub (Optional)
```yaml
GPG_PRIVATE_KEY: ASCII-armored GPG private key
GPG_PASSPHRASE: Passphrase for the GPG key
```

#### Creating GPG Key
```bash
# Generate key
gpg --full-generate-key

# Export private key
gpg --export-secret-keys --armor your-email@example.com > private.asc

# Export public key (distribute this)
gpg --export --armor your-email@example.com > public.asc
```

#### Local Testing
```bash
# Sign AppImage
gpg --detach-sign --armor BunkerVerseControlCenter.AppImage

# Verify
gpg --verify BunkerVerseControlCenter.AppImage.asc BunkerVerseControlCenter.AppImage
```

## GitHub Secrets Setup

### Adding Secrets to GitHub Repository

1. Go to Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Add each required secret

### Secret Naming Convention
- `WINDOWS_*`: Windows signing secrets
- `MACOS_*` / `APPLE_*`: macOS signing secrets
- `GPG_*`: Linux GPG signing (optional)

## Certificate Management

### Security Best Practices

1. **Never commit certificates to the repository**
2. **Use separate certificates for different environments** (dev/staging/production)
3. **Rotate certificates before expiration**
4. **Limit access to certificate passwords**
5. **Use hardware tokens for EV certificates when possible**

### Certificate Expiration Monitoring

Set up calendar reminders for:
- 30 days before expiration
- 60 days before expiration
- 90 days before expiration

### Certificate Renewal Process

#### Windows
1. Purchase renewal from certificate authority
2. Generate new .pfx file
3. Update `WINDOWS_CERT_BASE64` secret
4. Test signing in CI

#### macOS
1. Generate new certificate in Apple Developer portal
2. Download and export as .p12
3. Update `MACOS_CERT_BASE64` secret
4. Test signing and notarization in CI

## Troubleshooting

### Windows Issues

#### "Certificate not found"
- Verify the base64 encoding is correct
- Check certificate password
- Ensure certificate hasn't expired

#### "Signature verification failed"
- Check timestamp server is accessible
- Verify certificate chain is complete
- Ensure using SHA256 for signing

### macOS Issues

#### "Unable to find identity"
- Certificate might not be for "Developer ID Application"
- Keychain access issues
- Certificate not properly imported

#### "Notarization failed"
- Check app-specific password is correct
- Verify Team ID matches certificate
- Ensure all binaries in bundle are signed
- Check for hardened runtime issues

#### "The staple and validate action failed"
- Wait for notarization to complete
- Check network connectivity
- Verify notarization was successful

### Linux Issues

#### "GPG signature verification failed"
- Check GPG key hasn't expired
- Verify public key is distributed correctly
- Ensure GPG version compatibility

## Testing Code Signing

### Automated Tests

The CI pipeline automatically verifies signatures after signing:
- Windows: `signtool verify`
- macOS: `codesign --verify` and `spctl`
- Linux: `gpg --verify`

### Manual Verification

#### Windows
1. Right-click installer → Properties → Digital Signatures
2. Verify publisher name and timestamp

#### macOS
1. Right-click app → Open (first time)
2. Check for Gatekeeper warnings
3. Verify in System Preferences → Security & Privacy

#### Linux
```bash
# Download public key
curl -O https://bunkerverse.io/signing-key.asc
gpg --import signing-key.asc

# Verify signature
gpg --verify BunkerVerseControlCenter.AppImage.asc
```

## CI/CD Integration

### GitHub Actions Workflow

The `client-release.yml` workflow handles:
1. Building installers for all platforms
2. Importing certificates from secrets
3. Signing binaries and installers
4. Notarizing macOS builds
5. Creating GPG signatures for Linux
6. Generating checksums
7. Creating GitHub releases

### Manual Release Process

If automated signing fails:
1. Download unsigned artifacts from CI
2. Sign locally using scripts in `client/installer/*/`
3. Upload signed artifacts to GitHub release

## Compliance and Audit

### Audit Log

Maintain a log of:
- Certificate issuance/renewal dates
- Signing operations for releases
- Certificate access (who, when, why)
- Any signing failures or issues

### Compliance Requirements

- **Windows**: Follow Microsoft Authenticode requirements
- **macOS**: Comply with Apple notarization requirements
- **Linux**: Follow distribution-specific guidelines if applicable

## Support and Resources

### Documentation
- [Windows Authenticode](https://docs.microsoft.com/en-us/windows/win32/seccrypto/authenticode)
- [Apple Code Signing](https://developer.apple.com/documentation/security/code_signing_services)
- [Apple Notarization](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [GPG Signing](https://www.gnupg.org/documentation/)

### Certificate Authorities
- DigiCert (Windows EV certificates)
- Sectigo/Comodo (Standard certificates)
- Apple Developer Program (macOS certificates)

### Tools
- Windows: signtool.exe (Windows SDK)
- macOS: codesign, notarytool, stapler (Xcode)
- Linux: gpg (GnuPG)

## Contact

For certificate management and signing issues:
- DevOps Team: devops@bunkerverse.io
- Security Team: security@bunkerverse.io