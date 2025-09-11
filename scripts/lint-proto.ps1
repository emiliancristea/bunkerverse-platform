# Protocol Buffer Linting Script (Windows PowerShell)
# Runs comprehensive validation and linting for all .proto files

param(
    [switch]$Install,
    [switch]$Generate,
    [switch]$SkipBreaking
)

$ErrorActionPreference = "Stop"

Write-Host "🔍 Running Protocol Buffer static analysis..." -ForegroundColor Cyan

# Check if buf is installed
if (-not (Get-Command buf -ErrorAction SilentlyContinue)) {
    if ($Install) {
        Write-Host "📦 Installing buf CLI..." -ForegroundColor Yellow
        $bufVersion = "v1.28.1"
        $bufUrl = "https://github.com/bufbuild/buf/releases/download/$bufVersion/buf-Windows-x86_64.exe"
        Invoke-WebRequest -Uri $bufUrl -OutFile "buf.exe"
        Move-Item "buf.exe" "$env:USERPROFILE\bin\buf.exe" -Force
        $env:PATH += ";$env:USERPROFILE\bin"
    } else {
        Write-Host "❌ buf CLI not found. Run with -Install flag to install automatically." -ForegroundColor Red
        exit 1
    }
}

# Validate buf.yaml configuration
Write-Host "📋 Validating buf configuration..." -ForegroundColor Blue
buf config validate

# Run linting
Write-Host "🧹 Linting Protocol Buffer files..." -ForegroundColor Green
buf lint

# Check for breaking changes (if previous version exists)
if ((Test-Path "buf.lock") -and (-not $SkipBreaking)) {
    Write-Host "🔄 Checking for breaking changes..." -ForegroundColor Yellow
    buf breaking --against buf.lock
}

# Generate code from protobuf schemas
if ($Generate) {
    Write-Host "🔧 Generating code from Protocol Buffer schemas..." -ForegroundColor Magenta
    buf generate
    
    # Validate generated Rust code compiles
    Write-Host "🔬 Validating generated Rust code compiles..." -ForegroundColor DarkBlue
    Set-Location "libs\common-rust"
    cargo check
    Set-Location "..\\.."
}

Write-Host "✅ Protocol Buffer validation completed successfully!" -ForegroundColor Green