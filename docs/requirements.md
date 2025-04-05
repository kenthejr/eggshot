# Project Requirements

## Technical Constraints

- **Engine:** Bevy 0.15.3
- **Target:** Desktop (Windows, macOS, Linux)
- **Architecture:** Entity-Component-System (ECS)
- **Simulation:** Fully deterministic and event-driven
- **Networking:** LAN peer-to-peer for MVP, evolving to Tashi Consensus Engine
- **Responsiveness:** Input prediction and rollback-based netcode

## Simulation Requirements

- Use fixed time steps (`FixedUpdate`)
- Eliminate nondeterministic behavior (e.g., random `f32` values, system time)
- Deterministic RNG (e.g., seedable Xoshiro or LCG)
- Avoid non-deterministic APIs or native floating-point divergence
- All player-driven events must be serializable and re-playable

## Networking Requirements

- **Dedicated `network` system** handles game event buffering and stream processing
- Events flow through three paths:
  - **Outbox:** Player’s local events pushed into a pending outbound queue
  - **In Stream:** Optimistic, immediate messages from peers or TCE
  - **Final Stream:** Authoritative, consensus-ordered stream from TCE
- Game simulation is driven by these streams to allow prediction and rollback
- Networking plugin should register required systems and resources, but logic lives in `network/system.rs`
