# Generate comprehensive API documentation from Protocol Buffer schemas
# Creates HTML, OpenAPI, and Markdown documentation with versioning

param(
    [string]$Version = "v0.3.0"
)

$OutputDir = "docs\generated"

Write-Host "📚 Generating API documentation for version $Version..." -ForegroundColor Cyan

# Ensure output directories exist
New-Item -ItemType Directory -Force -Path "$OutputDir\protobuf", "$OutputDir\openapi", "$OutputDir\markdown", "$OutputDir\static" | Out-Null

# Generate documentation using buf
Write-Host "🔧 Generating Protocol Buffer documentation..." -ForegroundColor Blue
buf generate --template buf.gen.docs.yaml

# Create custom CSS file
$CustomCSS = @"
:root {
  --bunker-primary: #1a1a2e;
  --bunker-secondary: #16213e;
  --bunker-accent: #0f3460;
  --bunker-highlight: #e94560;
  --bunker-text: #f5f5f5;
}

body {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background: linear-gradient(135deg, var(--bunker-primary) 0%, var(--bunker-secondary) 100%);
  color: var(--bunker-text);
  margin: 0;
  padding: 0;
}

.header {
  background: var(--bunker-accent);
  padding: 20px;
  border-bottom: 2px solid var(--bunker-highlight);
}

.header h1 {
  color: var(--bunker-highlight);
  margin: 0;
  font-size: 2.5rem;
  font-weight: bold;
}

.version-badge {
  background: var(--bunker-highlight);
  color: white;
  padding: 4px 12px;
  border-radius: 15px;
  font-size: 0.8rem;
  font-weight: bold;
  margin-left: 15px;
}

.service-section {
  background: rgba(255,255,255,0.05);
  margin: 20px 0;
  padding: 20px;
  border-radius: 8px;
  border-left: 4px solid var(--bunker-highlight);
}

.method-card {
  background: rgba(255,255,255,0.03);
  margin: 10px 0;
  padding: 15px;
  border-radius: 6px;
  border: 1px solid rgba(233,69,96,0.3);
}

code {
  background: var(--bunker-primary);
  padding: 2px 6px;
  border-radius: 4px;
  color: var(--bunker-highlight);
  font-family: 'Consolas', 'Monaco', monospace;
}

pre {
  background: var(--bunker-primary);
  padding: 15px;
  border-radius: 6px;
  overflow-x: auto;
  border-left: 4px solid var(--bunker-highlight);
}
"@

Set-Content -Path "$OutputDir\static\custom.css" -Value $CustomCSS

# Generate comprehensive index page
$CurrentDate = Get-Date -Format "yyyy-MM-dd HH:mm:ss UTC"
$IndexHTML = @"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BUNKERVERSE Platform API Documentation $Version</title>
    <link rel="stylesheet" href="static/custom.css">
    <style>
        .nav-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }
        .doc-card {
            background: rgba(255,255,255,0.08);
            padding: 25px;
            border-radius: 10px;
            border: 1px solid rgba(233,69,96,0.2);
            transition: all 0.3s ease;
            text-decoration: none;
            color: inherit;
        }
        .doc-card:hover {
            transform: translateY(-5px);
            border-color: var(--bunker-highlight);
            box-shadow: 0 10px 20px rgba(233,69,96,0.2);
        }
        .card-title {
            color: var(--bunker-highlight);
            font-size: 1.4rem;
            margin-bottom: 10px;
        }
        .card-description {
            color: #ccc;
            line-height: 1.6;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>BUNKERVERSE Platform<span class="version-badge">$Version</span></h1>
        <p>Comprehensive API Documentation & Schema Reference</p>
    </div>
    
    <div style="padding: 40px;">
        <div class="nav-grid">
            <a href="protobuf/index.html" class="doc-card">
                <div class="card-title">📋 Protocol Buffer Schemas</div>
                <div class="card-description">
                    Complete reference for all protobuf message types, enums, and service definitions.
                    Includes field specifications, validation rules, and usage examples.
                </div>
            </a>
            
            <a href="openapi/index.html" class="doc-card">
                <div class="card-title">🌐 OpenAPI/Swagger Specs</div>
                <div class="card-description">
                    Interactive API documentation with request/response examples, authentication flows,
                    and testing capabilities for all gRPC services.
                </div>
            </a>
            
            <a href="markdown/README.md" class="doc-card">
                <div class="card-title">📖 Markdown Reference</div>
                <div class="card-description">
                    Developer-friendly markdown documentation suitable for README files,
                    wikis, and integration into documentation systems.
                </div>
            </a>
        </div>
        
        <div class="service-section">
            <h2>🏗️ Architecture Overview</h2>
            <p>The BUNKERVERSE platform consists of four primary gRPC services:</p>
            <ul>
                <li><strong>Identity Service</strong>: Player authentication, registration, and profile management</li>
                <li><strong>Account Service</strong>: Player stats, inventory, achievements, and progression</li>
                <li><strong>Marketplace Service</strong>: NFT trading, pricing, and transaction management</li>
                <li><strong>Indexer Service</strong>: Blockchain event processing and data synchronization</li>
            </ul>
        </div>
        
        <div class="service-section">
            <h2>🔧 Development Resources</h2>
            <ul>
                <li><a href="../protobuf-setup.md">Protobuf Setup Guide</a></li>
                <li><a href="../../schemas/proto/">Raw Proto Files</a></li>
                <li><a href="../../buf.yaml">Buf Configuration</a></li>
                <li><a href="../../Makefile">Development Commands</a></li>
            </ul>
        </div>
        
        <footer style="margin-top: 50px; padding-top: 20px; border-top: 1px solid rgba(255,255,255,0.1); text-align: center; color: #888;">
            <p>Generated on $CurrentDate | Version $Version | BUNKERVERSE Platform</p>
        </footer>
    </div>
</body>
</html>
"@

Set-Content -Path "$OutputDir\index.html" -Value $IndexHTML

# Generate OpenAPI aggregated spec
Write-Host "📊 Creating aggregated OpenAPI specification..." -ForegroundColor Green
$OpenAPISpec = @"
openapi: 3.0.3
info:
  title: BUNKERVERSE Platform API
  version: $Version
  description: |
    Comprehensive API documentation for the BUNKERVERSE gaming platform.
    
    ## Overview
    The BUNKERVERSE platform provides a complete Web3 gaming infrastructure with:
    - Player identity and authentication
    - NFT-based asset management  
    - Decentralized marketplace
    - Blockchain event indexing
    
    ## Authentication
    All services require JWT-based authentication with player identity validation.
    
    ## Rate Limiting  
    API endpoints are rate-limited per player:
    - Standard operations: 100 requests/minute
    - Trading operations: 10 requests/minute
    - Administrative operations: 5 requests/minute
    
  contact:
    name: BUNKERVERSE Development Team
    url: https://bunkerverse.com
    email: dev@bunkerverse.com
  license:
    name: MIT
    url: https://opensource.org/licenses/MIT

servers:
  - url: https://api.bunkerverse.com/v1
    description: Production API
  - url: https://staging-api.bunkerverse.com/v1  
    description: Staging API
  - url: http://localhost:8080/v1
    description: Local Development

paths:
  # Paths will be generated from protobuf definitions
  
components:
  securitySchemes:
    BearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
      description: JWT token obtained from identity service
      
  schemas:
    # Schemas will be generated from protobuf definitions

security:
  - BearerAuth: []

tags:
  - name: Identity
    description: Player authentication and profile management
  - name: Account  
    description: Player stats, inventory, and progression
  - name: Marketplace
    description: NFT trading and marketplace operations
  - name: Indexer
    description: Blockchain event processing and queries
"@

Set-Content -Path "$OutputDir\openapi\bunkerverse-api-$Version.yaml" -Value $OpenAPISpec

Write-Host "✅ API documentation generation completed!" -ForegroundColor Green
Write-Host "📍 Documentation available at: $OutputDir\index.html" -ForegroundColor Yellow
Write-Host "🌐 OpenAPI spec available at: $OutputDir\openapi\bunkerverse-api-$Version.yaml" -ForegroundColor Yellow