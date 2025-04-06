# Eggshot

<p align="center">
  <img src="assets/images/eggshot_logo.png" alt="Eggshot Logo" width="300"/>
</p>

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2021_edition-blue.svg)](https://www.rust-lang.org)
[![Bevy](https://img.shields.io/badge/Bevy-0.15.3-orange.svg)](https://bevyengine.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/eggshot/actions)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

</div>

A 3D first-person shooter (FPS) arena game where players control egg-shaped characters in intense multiplayer combat. Inspired by *Shell Shockers*, this project is built using **Bevy 0.15.3** and targets desktop platforms.

For a detailed overview of the project, see [docs/overview.md](docs/overview.md).

## Features

- Deterministic simulation suitable for synchronization via the **Tashi Consensus Engine (TCE)**
- Fast-paced, rollback-driven multiplayer gameplay
- Modular ECS-based architecture for scalability and clarity
- LAN support with lobby creation and ready-up system

## Technical Details

- **Engine:** Bevy 0.15.3
- **Target Platforms:** Desktop (Windows, macOS, Linux)
- **Architecture:** Entity-Component-System (ECS)
- **Simulation:** Fully deterministic and event-driven
- **Networking:** LAN peer-to-peer for MVP, evolving to Tashi Consensus Engine
- **Responsiveness:** Input prediction and rollback-based netcode

## Project Structure

```
eggshot/
├── assets/                      # Game assets
│   ├── images/                  # Game images and logos
│   ├── models/                  # 3D models
│   ├── textures/                # Texture files
│   └── sounds/                  # Sound effects and music
├── docs/                        # Project documentation
├── src/                         # Game source code
│   ├── main.rs                  # Entry point
│   ├── app.rs                   # Bevy app setup and state configuration
│   ├── config/                  # Game-wide constants, settings, and tuning
│   ├── core/                    # Deterministic simulation, rollback, time
│   ├── entities/                # Entity definitions and components
│   ├── systems/                 # ECS systems for game behavior
│   ├── plugins/                 # Modular Bevy plugins (feature gates)
│   ├── lobby/                   # Lobby discovery, creation, join, ready-up
│   ├── resources/               # Global resources: timers, game state, etc.
│   └── network/                 # Deterministic network I/O and buffering
├── Cargo.toml                   # Project manifest and dependencies
├── LICENSE                      # MIT License
├── CONTRIBUTING.md              # Contribution guidelines
└── README.md                    # This file
```

## Development Roadmap

### Phase 1: Core Gameplay (Single-Player)
- [ ] FPS movement and camera
- [ ] Simple level
- [ ] Basic gun mechanics
- [ ] Enemy AI (dummy for testing)
- [ ] Health and respawn system
- [ ] Game UI (health, ammo, crosshair)

### Phase 2: LAN Multiplayer
- [ ] Peer discovery via LAN broadcast
- [ ] Lobby system (host/join)
- [ ] "Ready Up" and start synchronization
- [ ] Input prediction and basic rollback
- [ ] Synchronize simulation over LAN with minimal desync

### Phase 3: Online Multiplayer
- [ ] Integrate Tashi Consensus Engine
- [ ] Replace LAN messaging with TCE stream
- [ ] Test fully deterministic multiplayer with TCE
- [ ] Implement authoritative rollback state resolution

### Phase 4: Polish & Expansion
- [ ] Add sounds, effects, and animations
- [ ] Improved maps and level design tools
- [ ] Egg customization system
- [ ] Support for more than two players

## Building and Running

```bash
# Build the project
cargo build

# Run in debug mode
cargo run

# Run in release mode
cargo run --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests. 