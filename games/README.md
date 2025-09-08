# Game Servers & UE5 Integration

Unreal Engine 5 game servers and blockchain integration plugin.

## Components

- `ue5-plugin/` - Netchain plugin for UE5 blockchain integration
- `constructs/arena_site_mvp/` - Arena Site game server (MVP)
- `constructs/frontier_island_mvp/` - Frontier Island game server (MVP)  
- `constructs/synthesis_citadel_mvp/` - Synthesis Citadel game server (MVP)
- `constructs/stubs/` - Placeholder projects for remaining constructs

## Requirements

- Unreal Engine 5.3+
- MSVC 2022 compiler
- Agones for Kubernetes orchestration (production)

## Development

Each construct is a complete UE5 project with:
- Custom game modes and player controllers
- Netchain plugin integration for L3 blockchain features
- Agones SDK for multiplayer server lifecycle management
- Real-time communication with platform services

Build instructions are project-specific - see individual construct READMEs.