# Jumping Bird - Game Documentation

## Overview

**Jumping Bird** is a terminal-based arcade game built with Rust and the ratatui TUI library. The game features a bird that must navigate through obstacles while using a limited jump system. The goal is to survive as long as possible while avoiding collisions with trees, rocks, and clouds.

## Game Features

- 🐤 **Character Control**: Control a bird with a 3-jump mechanic per ground contact
- 🌲 **Dynamic Obstacles**: Three types of obstacles spawn randomly:
  - Trees (🌲) - Ground level
  - Rocks (◆) - Ground level
  - Clouds (☁) - Mid-air
- 📊 **Scoring System**: Earn points for survival time
- 🎨 **Terminal UI**: Colorful, responsive terminal interface using ratatui
- ⌨️ **Simple Controls**: Space bar to jump, Q to quit

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [https://rustup.rs/](https://rustup.rs/))
- macOS, Linux, or Windows with a compatible terminal

### Installation

```bash
# Clone or navigate to the project directory
cd jumping-bird

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### First Game

1. Launch the game with `cargo run`
2. Press **Space** to make your bird jump
3. Avoid obstacles by jumping over trees and rocks or under clouds
4. Survive as long as possible to increase your score
5. Press **Q** to quit when game is over

## Game Mechanics

### Bird Physics

The bird is governed by simple physics simulation:

- **Gravity**: Continuously pulls the bird downward at 0.2 pixels/frame²
- **Jump**: Pressing space applies upward velocity (-4 pixels/frame)
- **Terminal Velocity**: Bird accelerates down until hitting ground
- **Jump Counter**: Resets to 3 when the bird lands on the ground

### Jump System

- **3 Jumps Per Landing**: After touching the ground, you get 3 mid-air jumps
- **Progressive Momentum**: Each jump uses the same power, but gravity affects timing
- **Air Control**: You can perform all 3 jumps while airborne
- **Strategic Jumping**: Use multiple jumps to adjust height and avoid obstacles

### Obstacles

Three obstacle types spawn randomly:

| Obstacle | Symbol | Height | Width | Strategy |
|----------|--------|--------|-------|----------|
| Tree | 🌲 | Ground | 3 chars | Jump over |
| Rock | ◆ | Ground | 2 chars | Jump over or squeeze through |
| Cloud | ☁ | Mid-air | 4 chars | Duck under or jump over |

Obstacles move from right to left at 1 character per frame.

### Scoring

- **Score Increment**: +1 point per frame (≈10 frames per second)
- **Display**: Score shown as "Score: X" where X is frames/10
- **Survival Metric**: Score represents survival time in deciseconds

### Collision Detection

Collision is calculated as axis-aligned bounding box (AABB):

```
Bird: 1x1 at position (x, y)
Obstacle: width x 1 at position (x, y)

Collision if:
  bird.right > obstacle.left AND
  bird.left < obstacle.right AND
  bird.bottom > obstacle.top AND
  bird.top < obstacle.bottom
```

## Controls

| Control | Action |
|---------|--------|
| **Space** | Jump (use up to 3 times per ground contact) |
| **Q** / **Esc** | Quit the game |

## Game States

### Playing
- Bird is alive and moving
- Obstacles spawn and move left
- Player can jump

### Game Over
- Collision with obstacle detected
- Game freezes on last frame
- Final score displayed
- Press Q to quit

## Technical Architecture

### Game State (`Game` struct)

```rust
pub struct Game {
    bird: Bird,           // Player character
    obstacles: Vec<Obstacle>,  // Active obstacles
    score: u32,           // Current score
    game_over: bool,      // Game state flag
    frame_count: usize,   // Frame counter
}
```

### Bird State (`Bird` struct)

```rust
pub struct Bird {
    x: u16,               // Horizontal position
    y: u16,               // Vertical position
    velocity: f32,        // Current vertical velocity
    jumps_left: u8,       // Remaining jumps (0-3)
    is_jumping: bool,     // Jump state flag
}
```

### Obstacle System

Each obstacle has:
- **Position (x, y)**: Coordinate on screen
- **Width**: Determines collision area
- **Type**: Visual representation and behavior

## Game Constants

Located at the top of `src/main.rs`:

```rust
const GAME_WIDTH: u16 = 80;        // Game area width
const GAME_HEIGHT: u16 = 20;       // Game area height
const BIRD_START_X: u16 = 5;       // Bird spawn X position
const BIRD_START_Y: u16 = 10;      // Bird spawn Y position (middle)
const MAX_JUMPS: u8 = 3;           // Jumps per landing
const JUMP_POWER: i16 = 4;         // Jump velocity
const GRAVITY: f32 = 0.2;          // Gravity acceleration
const OBSTACLE_SPEED: u16 = 1;     // Pixels/frame movement
const SPAWN_RATE: usize = 15;      // Frames between spawns
```

### Tuning Difficulty

Adjust these constants to change game difficulty:

- **Increase `GRAVITY`**: Game gets harder (0.2 → 0.3)
- **Decrease `SPAWN_RATE`**: More obstacles (15 → 10)
- **Decrease `JUMP_POWER`**: Shorter jumps (4 → 3)
- **Decrease `OBSTACLE_SPEED`**: Slower obstacles (1 → 0) - easier
- **Increase `OBSTACLE_SPEED`**: Faster obstacles (1 → 2) - harder

## Dependencies

- **ratatui** (0.30.0): Terminal UI framework
- **crossterm** (0.29.0): Cross-platform terminal control
- **color-eyre** (0.6.3): Error handling and color formatting

## File Structure

```
jumping-bird/
├── Cargo.toml              # Project manifest
├── src/
│   └── main.rs            # Game implementation
├── docs/
│   ├── README.md          # This file
│   ├── DEVELOPMENT.md     # Development guide
│   └── GAMEPLAY.md        # Detailed gameplay guide
└── target/
    └── debug/
        └── jumping-bird   # Compiled binary
```

## Performance

- **Frame Rate**: ~10 FPS (100ms per frame)
- **Rendering**: Per-frame terminal redraw
- **Memory**: Minimal (game state + obstacle vec)
- **CPU**: Low overhead, suitable for remote terminals

## Troubleshooting

### Game doesn't start
- Ensure terminal supports Unicode characters
- Try running in a different terminal emulator
- Check Rust installation: `rustc --version`

### Jumps don't respond
- Space key might be mapped differently on your keyboard
- Try pressing space when bird is on the ground
- Check if terminal is capturing key events

### Rendering looks wrong
- Ensure terminal window is at least 90x25 characters
- Terminal might not support emoji characters (🐤, 🌲, etc.)
- Try a terminal with better Unicode support (iTerm2, VS Code terminal, modern Windows Terminal)

### Game runs slowly
- Close other applications
- Try reducing terminal refresh rate or using a different terminal
- Check system load with `top` or `Activity Monitor`

## Future Enhancements

Potential features for future versions:

- [ ] Power-ups (shield, extra jump)
- [ ] Multiple difficulty levels
- [ ] High score persistence
- [ ] Sound effects
- [ ] Better graphics/animations
- [ ] Multiplayer modes
- [ ] Level progression
- [ ] Leaderboard system
- [ ] Configuration file support
- [ ] Different game modes

## Credits

- Game Design & Implementation: Shahryar Tarnasi
- Built with Rust, ratatui, and crossterm
- Inspired by classic arcade games

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Support

For issues, questions, or suggestions, please open an issue on the GitHub repository.

---

**Have fun jumping! 🐤**
