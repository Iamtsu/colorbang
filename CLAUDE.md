# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Color Bang! is a 2D arcade-style shooter game written in Rust using the speedy2d graphics library. Players shoot colorful enemies in an arena-style environment with wave-based progression.

## Build and Run Commands

```bash
# Build and run in debug mode (with optimization level 1)
cargo run

# Build and run in release mode (with full optimizations)
cargo run --release

# Build only
cargo build
cargo build --release

# Check for compilation errors without building
cargo check
```

Note: Debug builds use opt-level=1 for faster compilation while maintaining some performance. Dependencies are compiled with opt-level=3 even in debug mode.

## Architecture Overview

### Entity-Component System

The game uses a trait-based architecture centered around the `GameEntity` trait (src/game_entity.rs):

- **GameEntity trait**: Core interface implemented by all interactive game objects
  - `draw()`: Renders the entity
  - `update()`: Updates entity state, returns false to mark for removal
  - `collider_info()`: Returns collision metadata via `ColliderInfo` struct
  - `deal_damage()`: Handles collision response with physics impulse

### Entity Types

All entities implement `GameEntity`:

- **Player** (src/player.rs): Player-controlled entity with mouse/keyboard aiming
- **Enemy** (src/enemy.rs): AI-controlled entities that move toward the player and wrap around screen edges
- **Bullet** (src/bullet.rs): Projectiles with health-based lifetime
- **Particle** (src/particle.rs): Visual effects with alpha decay and drag physics

### Collision System

Layer-based collision using bitflags (defined in main.rs):
```rust
const COL_PLAYER: u8 = 0b00000001;
const COL_ENEMY: u8 = 0b00000010;
const COL_BULLET: u8 = 0b00000100;
```

Each entity has:
- `layer`: What collision layer it belongs to
- `mask`: Which layers it can collide with

The `collide()` function in src/game_entity.rs checks layer compatibility before performing circle-circle collision detection.

### Enemy-Enemy Collision Optimization

Enemy-enemy collisions use unsafe raw pointer arithmetic (src/main.rs:189-210) to achieve mutable aliasing for collision pairs. This avoids the borrow checker limitations when mutating two enemies simultaneously during collision response. The safe alternative using `split_at_mut()` is commented out above this section.

### Game Loop Structure

The main game loop in `MyWindowHandler::on_draw()` follows this pattern:

1. **Wave Management**: Spawn new enemies when none remain
2. **Super Bang Charging**: Convert super_bang points to charged_super_bang
3. **Bullet Firing**: Handle cooldown-based firing
4. **Player Aiming**: Mouse-based or keyboard rotation
5. **Update Phase**: Call `update()` on all entities
6. **Collision Detection**: Check player-enemy, bullet-enemy, and enemy-enemy collisions
7. **Draw Phase**: Render all entities and UI

Entities are retained based on their `update()` return value and spatial bounds checking.

### Audio System

The `SoundPlayer` (src/sound.rs) uses the kira audio library:
- Loads all .ogg files from the assets directory at startup
- Supports randomized sound selection using name prefixes (e.g., "Laser_07" to "Laser_09" for fire sounds)
- Maps `SoundType` enum to specific sound file patterns and volume levels

Sound files are stored in assets/ and loaded dynamically, so adding new sounds requires following the naming convention (e.g., "Laser_XX.ogg", "incoming_XX.ogg").

### Controls

- Mouse movement: Aim
- Left click / Space: Fire
- Right click / R: Charge super bang (hold) → Release for radial bullet burst
- W/S: Move forward/backward
- A/D: Rotate (when cursor hidden)
- Escape: Toggle cursor visibility/grab
- Backspace: Pause
- Middle click: Debug spawn 10 enemies

## Key Implementation Details

- **Physics**: Collision responses use momentum-based impulse calculations (see `impulse()` in game_entity.rs)
- **Screen Wrapping**: Enemies wrap around screen edges; bullets and player do not
- **Health System**: Player radius = health (visual and functional). Enemies shrink on damage.
- **Super Bang**: Fires radial burst of bullets with randomized angles and speeds
- **Bullet Lifetime**: Bullets have health that decrements each frame (600 for normal, 200 for super bang)
- **Performance Profile**: Dependencies are heavily optimized even in debug builds for acceptable frame rates

## File Structure

```
src/
├── main.rs           # Game loop, window handling, main game state
├── game_entity.rs    # Core trait and collision system
├── player.rs         # Player entity implementation
├── enemy.rs          # Enemy entity and spawn logic
├── bullet.rs         # Bullet entity and super bang
├── particle.rs       # Visual particle effects
└── sound.rs          # Audio system with kira

assets/
├── *.ogg            # Sound effects (Laser_XX, incoming_XX, part)
└── Nasa21-l23X.ttf  # Font for UI text
```
