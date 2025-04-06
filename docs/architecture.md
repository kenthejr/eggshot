# System Architecture

## Core Concepts

- **Deterministic Core:** Game logic is deterministic and decoupled from the rendering frame rate.
- **Rollback Support:** Systems must support prediction and rollback based on a timeline of events.
- **Modular Design:** Game features are grouped into reusable Bevy plugins.
- **Network Stream Decoupling:** Game events are not applied directly to the game state but flow through deterministic buffers for validation and rollback control.

## Networking Architecture

A dedicated `network` module implements the deterministic network event flow:




### Event Flow Breakdown

- **Outbox (local):**  
  - Captured input is wrapped as `GameEvent` and stored locally.
  - `network::io` sends it over LAN or to TCE.

- **In Stream (optimistic):**  
  - Events from peers or TCE are received and added to the `InStream`.
  - These are applied immediately for responsiveness.

- **Final Stream (authoritative):**  
  - Consensus-ordered events from TCE are added to the `FinalStream`.
  - Used for rollback and replay to ensure deterministic convergence.

### ECS Schedule Integration

- `network::system` runs during a `FixedUpdate` stage to:
  - Capture local player input
  - Push to `Outbox`
  - Poll `InStream` and `FinalStream`
  - Apply optimistic and finalized events to the simulation

## Deterministic Network Event Flow

Eggshot’s networking model is built around three distinct streams to support prediction, rollback, and deterministic convergence across clients.

### Event Streams

- **Outbox (Local → Outbound)**
  - Captures local player actions as `GameEvent`s.
  - Events are added to an `Outbox` buffer and sent via LAN or Tashi Consensus Engine (TCE).
  - Events must be timestamped or frame-indexed to preserve causal order.

- **In Stream (Unfinalized Inbound)**
  - Contains player actions from all peers as soon as they are received.
  - Used for **optimistic application**: applied immediately to give responsive feel.
  - May be rolled back later based on authoritative order.

- **Final Stream (Consensus-Ordered Inbound)**
  - The authoritative sequence of `GameEvent`s, finalized by TCE.
  - Triggers rollback and re-simulation when the final order differs from the optimistic application.

### Timing and Scheduling

- All network event processing occurs in `FixedUpdate` systems.
- Outbox is flushed each tick, and `InStream` / `FinalStream` are polled.
- Rollback-aware systems reconcile local state with finalized events.

### Buffers (Bevy Resources)

```rust
// src/network/buffers.rs

#[derive(Resource)]
pub struct Outbox(pub Vec<GameEvent>);

#[derive(Resource)]
pub struct InStream(pub VecDeque<GameEvent>);

#[derive(Resource)]
pub struct FinalStream(pub VecDeque<GameEvent>);
```

## File Structure

```
eggshot/
├── assets/                      # Game assets: models, textures, sounds
├── docs/                        # Project documentation
│   ├── overview.md
│   ├── requirements.md
│   ├── architecture.md
│   ├── roadmap.md
│   └── networking.md            # (Optional) In-depth explanation of network flow
├── src/                         # Game source code
│   ├── main.rs                  # Entry point
│   ├── app.rs                   # Bevy app setup and state configuration
│   ├── config/                  # Game-wide constants, settings, and tuning
│   ├── core/                    # Deterministic simulation, rollback, time
│   │   ├── simulation.rs
│   │   └── rollback.rs
│   ├── entities/                # Entity definitions and components
│   │   ├── player.rs
│   │   └── enemy.rs
│   ├── systems/                 # ECS systems for game behavior
│   │   ├── movement.rs
│   │   ├── shooting.rs
│   │   └── health.rs
│   ├── plugins/                 # Modular Bevy plugins (feature gates)
│   │   ├── input_plugin.rs
│   │   ├── networking_plugin.rs
│   │   └── ui_plugin.rs
│   ├── lobby/                   # Lobby discovery, creation, join, ready-up
│   │   ├── lobby.rs
│   │   └── net_discovery.rs
│   ├── resources/               # Global resources: timers, game state, etc.
│   └── network/                 # Deterministic network I/O and buffering
│       ├── mod.rs
│       ├── events.rs            # GameEvent, PlayerAction definitions
│       ├── buffers.rs           # Outbox, InStream, FinalStream resources
│       ├── io.rs                # Networking transport (LAN or TCE)
│       └── system.rs            # Bevy system for event flow management
├── Cargo.toml                   # Project manifest and dependencies
└── README.md                    # Entry point for project context
```

### Key Modules

- `core/`: Deterministic simulation + rollback logic
- `entities/`: Definitions for players, enemies, bullets, etc.
- `systems/`: ECS systems like input, movement, shooting, and health
- `plugins/`: Reusable feature modules (e.g., input, UI, netcode)
- `lobby/`: LAN discovery, matchmaking, and match coordination

### Determinism Practices

- Avoid `f32` in logic critical to simulation; prefer fixed-point or validated `f64`
- Use custom deterministic RNG seeded from lobby state
- Deterministic input processing and event queueing
