# Color Bang!

A fast-paced 2D arcade shooter written in Rust where you battle waves of colorful enemies in an arena-style environment.

## Overview

Color Bang! is an intense top-down shooter where you control a white circle navigating through increasingly difficult waves of colorful enemies. Survive by shooting enemies, managing your health, and strategically using super bang attacks to clear the screen.

## Features

- **Wave-based gameplay**: Face progressively larger waves of enemies
- **Physics-based combat**: Enemies and bullets interact with momentum-based collisions
- **Super Bang system**: Charge up powerful radial bullet bursts
- **Dynamic particle effects**: Visual feedback for all collisions
- **Sound effects**: Randomized laser and explosion sounds for variety
- **Statistics tracking**: Monitor your accuracy with hit/miss counters

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))

### Building

```bash
# Clone the repository
git clone https://github.com/Iamtsu/colorbang.git
cd colorbang

# Build and run (debug mode with optimizations)
cargo run

# Build and run (release mode for best performance)
cargo run --release
```

## How to Play

### Controls

**Mouse Controls:**
- Move mouse: Aim direction
- Left click: Fire bullets
- Right click (hold): Charge super bang → Release for radial burst
- Escape: Toggle cursor visibility/lock

**Keyboard Controls:**
- W: Move forward
- S: Move backward
- A: Rotate left (when cursor is hidden)
- D: Rotate right (when cursor is hidden)
- Space: Fire bullets
- R (hold): Charge super bang → Release for radial burst
- Backspace: Pause game

**Debug:**
- Middle mouse: Spawn 10 enemies

### Game Mechanics

- **Health System**: Your size represents your health - you shrink when hit by enemies
- **Super Bang**: Earned after clearing waves. Hold right-click/R to charge, release to fire a radial burst of bullets
- **Enemy Behavior**: Enemies spawn around you and move toward your position, wrapping around screen edges
- **Collisions**: Enemies damage each other and the player on contact, with physics-based knockback

### Objectives

- Survive waves of enemies
- Maximize accuracy (tracked in the HUD)
- Build up and strategically use super bangs
- Keep your health (size) above zero

## Technical Details

### Built With

- [speedy2d](https://github.com/QuantumBadger/Speedy2D) - Fast 2D graphics library
- [kira](https://github.com/tesselode/kira) - Audio playback
- [rand](https://github.com/rust-random/rand) - Random number generation

### Performance

The game uses aggressive optimization even in debug builds:
- Debug mode: opt-level 1 for the game, opt-level 3 for dependencies
- Release mode: Full optimizations with debug info stripped

### Architecture

The game uses a trait-based entity system where all interactive objects (Player, Enemy, Bullet, Particle) implement the `GameEntity` trait. Collisions are handled via a layer-mask system using bitflags for efficient filtering.

## Release with `release-vX` tags

```bash
# create a release tag 
git tag -a release-v1.4 -m "release 1.4, gogogo"

# push the tag to origin
git push origin release-v1.4

```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to submit pull requests.

**Important:** All PRs must use semantic branch naming:
- `feature/*` for new features
- `bugfix/*` for bug fixes

## License

See LICENSE file for details.

## Credits

- Font: Nasa21-l23X.ttf (https://www.dafont.com/nasa21.font)
- Sound effects: Laser and incoming wave sounds (assets/*.ogg)
