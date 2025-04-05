# Development Roadmap

## Phase 1: Core Gameplay (Single-Player)

- [ ] FPS movement and camera
- [ ] Simple level
- [ ] Basic gun mechanics
- [ ] Enemy AI (dummy for testing)
- [ ] Health and respawn system
- [ ] Game UI (health, ammo, crosshair)

## Phase 2: LAN Multiplayer

- [ ] Peer discovery via LAN broadcast
- [ ] Lobby system (host/join)
- [ ] "Ready Up" and start synchronization
- [ ] Input prediction and basic rollback
- [ ] Synchronize simulation over LAN with minimal desync

## Phase 3: Online Multiplayer

- [ ] Integrate Tashi Consensus Engine
- [ ] Replace LAN messaging with TCE stream
- [ ] Test fully deterministic multiplayer with TCE
- [ ] Implement authoritative rollback state resolution

## Phase 4: Polish & Expansion

- [ ] Add sounds, effects, and animations
- [ ] Improved maps and level design tools
- [ ] Egg customization system
- [ ] Support for more than two players
