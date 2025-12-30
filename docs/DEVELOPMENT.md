# Jumping Bird - Development Guide

## Architecture Overview

This document describes the technical implementation of Jumping Bird for developers.

## Project Structure

```
src/main.rs
├── Constants (Game parameters)
├── Bird (Entity struct & impl)
├── Obstacle (Entity struct & impl)
├── Game (State management)
├── Rendering (UI components)
└── Main Loop (Event handling & timing)
```

## Core Components

### 1. Constants

Game parameters defined at module level:

```rust
const GAME_WIDTH: u16 = 80;        // Playfield width in characters
const GAME_HEIGHT: u16 = 20;       // Playfield height in characters
const BIRD_START_X: u16 = 5;       // Bird initial X position
const BIRD_START_Y: u16 = 10;      // Bird initial Y position
const MAX_JUMPS: u8 = 3;           // Jumps granted per landing
const JUMP_POWER: i16 = 4;         // Initial jump velocity
const GRAVITY: f32 = 0.2;          // Gravity acceleration per frame
const OBSTACLE_SPEED: u16 = 1;     // Pixels obstacle moves per frame
const SPAWN_RATE: usize = 15;      // Frames between obstacle spawns
```

**Design Decision**: Constants in code allow recompilation for tuning. For production, consider `serde` for config files.

### 2. Bird Entity

```rust
#[derive(Clone, Copy, Debug)]
struct Bird {
    x: u16,              // Horizontal position
    y: u16,              // Vertical position
    velocity: f32,       // Current vertical velocity
    jumps_left: u8,      // Remaining jumps (0-3)
    is_jumping: bool,    // Currently in jump state
}
```

#### Bird Methods

**`new()`**: Creates bird at starting position
```rust
Bird {
    x: BIRD_START_X,
    y: BIRD_START_Y,
    velocity: 0.0,
    jumps_left: MAX_JUMPS,
    is_jumping: false,
}
```

**`jump()`**: Executes a jump if jumps remain
```rust
fn jump(&mut self) {
    if self.jumps_left > 0 {
        self.velocity = -(JUMP_POWER as f32);
        self.jumps_left -= 1;
        self.is_jumping = true;
    }
}
```

**`update()`**: Physics simulation per frame
1. Apply gravity: `velocity += GRAVITY`
2. Update position: `y += velocity`
3. Check ground collision: If Y >= ground, reset to ground
4. Restore jumps on landing: `jumps_left = MAX_JUMPS`

**Physics Simulation**:
```
Frame 0: y=10, v=0
Jump: v=-4
Frame 1: v=-3.8, y=6.2
Frame 2: v=-3.6, y=2.6
Frame 3: v=-3.4, y=-0.8 (clamped to 0, velocity reset)
Frame 4: v=0.2, y=0.2
Frame 5: v=0.4, y=0.6
...
Frame N: y=ground → jumps_left restored to 3
```

### 3. Obstacle Entity

```rust
#[derive(Clone, Debug)]
struct Obstacle {
    x: u16,                  // Horizontal position
    y: u16,                  // Vertical position
    width: u16,              // Collision width
    obstacle_type: ObstacleType,
}

#[derive(Clone, Debug, PartialEq)]
enum ObstacleType {
    Tree,
    Rock,
    Cloud,
}
```

#### Obstacle Methods

**`new(x, y, type)`**: Factory constructor
- Tree: width=3
- Rock: width=2
- Cloud: width=4

**`update()`**: Movement logic
```rust
if self.x > 0 {
    self.x -= OBSTACLE_SPEED;
}
```

**`is_off_screen()`**: Determines if obstacle should be removed
```rust
self.x + self.width < 1  // Completely off left edge
```

**`collides_with_bird()`**: AABB collision detection
```rust
fn collides_with_bird(&self, bird: &Bird) -> bool {
    // Check if bounding boxes overlap
    bird_right > obstacle_left &&
    bird_left < obstacle_right &&
    bird_bottom > obstacle_top &&
    bird_top < obstacle_bottom
}
```

**Design Decision**: AABB chosen for:
- O(1) collision checking
- Accurate enough for pixel-art style game
- Simple to debug and visualize

### 4. Game State

```rust
struct Game {
    bird: Bird,
    obstacles: Vec<Obstacle>,
    score: u32,
    game_over: bool,
    frame_count: usize,
}
```

#### Game Methods

**`new()`**: Initialize fresh game state
```rust
Game {
    bird: Bird::new(),
    obstacles: Vec::new(),
    score: 0,
    game_over: false,
    frame_count: 0,
}
```

**`handle_input()`**: Event processing
```rust
match key_code {
    KeyCode::Char(' ') => self.bird.jump(),
    KeyCode::Char('q') | KeyCode::Esc => self.game_over = true,
    _ => {},
}
```

**`update()`**: Single frame update
```
1. Update bird physics
2. Update obstacles (move left)
3. Remove off-screen obstacles
4. Spawn new obstacles (every SPAWN_RATE frames)
5. Check collisions
6. Increment score
7. Increment frame count
```

Obstacle Spawning Pattern:
```rust
if self.frame_count % SPAWN_RATE == 0 {
    let obstacle_type = match self.frame_count / SPAWN_RATE % 3 {
        0 => ObstacleType::Tree,
        1 => ObstacleType::Rock,
        _ => ObstacleType::Cloud,
    };
    // Clouds spawn mid-air, others on ground
    let y = if obstacle_type == ObstacleType::Cloud { 3 } else { 19 };
    self.obstacles.push(Obstacle::new(GAME_WIDTH, y, obstacle_type));
}
```

### 5. Rendering

**Architecture**: Draw-to-buffer pattern
```
1. Create 2D character buffer (GAME_HEIGHT × GAME_WIDTH)
2. Populate buffer with game entities
3. Convert buffer rows to ratatui Lines
4. Render with Paragraph widget
```

**Layers** (back to front):
1. Ground line (═)
2. Obstacles (🌲, ◆, ☁)
3. Bird (🐤)
4. UI elements (title, score, instructions)

**Key Function: `render_game_area()`**
```rust
fn render_game_area(frame: &mut Frame, game: &Game, game_area: Rect) {
    // Initialize empty buffer
    let mut buffer = vec![vec![' '; GAME_WIDTH as usize]; GAME_HEIGHT as usize];
    
    // Draw ground
    for x in 0..GAME_WIDTH {
        buffer[GAME_HEIGHT-1][x] = '=';
    }
    
    // Draw bird
    buffer[bird.y][bird.x] = '🐤';
    
    // Draw obstacles
    for obstacle in game.obstacles {
        // Draw with appropriate symbol and width
    }
    
    // Convert to Lines and render
    let lines = buffer.iter().map(|row| Line::from(...)).collect();
    frame.render_widget(Paragraph::new(lines), game_area);
}
```

**Why Buffer Pattern?**
- Decouples entity state from rendering
- Simplifies 2D positioning
- Easy to extend with visual effects
- Clear separation of concerns

### 6. Main Event Loop

```rust
fn app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut game = Game::new();
    let tick_rate = Duration::from_millis(100);  // ~10 FPS
    let mut last_tick = Instant::now();

    loop {
        // Render current state
        terminal.draw(|frame| render(frame, &game))?;

        // Input handling with timeout
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                game.handle_input(key.code);
            }
        }

        // Game logic update
        if last_tick.elapsed() >= tick_rate {
            game.update();
            last_tick = Instant::now();
        }

        // Exit condition
        if game.game_over {
            wait_for_quit();
            return Ok(());
        }
    }
}
```

**Timing Strategy**:
- Target ~10 FPS for readable gameplay
- `poll()` with timeout: Non-blocking input
- Decoupled from rendering for consistency
- Frame rate independent of input latency

## Data Flow

```
Event System
    ↓
poll(timeout) ← Waits for input or tick
    ↓
[Input] → handle_input() → bird.jump()
           OR
[Timeout] → update() → Physics, Obstacles, Collisions
    ↓
render() → buffer population → ratatui widgets
    ↓
Terminal Display
```

## Collision System

### AABB (Axis-Aligned Bounding Box)

Each entity has implicit bounds:
- **Bird**: 1×1 at (bird.x, bird.y)
- **Obstacle**: width×1 at (obstacle.x, obstacle.y)

### Collision Check

```rust
fn collides_with_bird(&self, bird: &Bird) -> bool {
    let bird_left = bird.x;
    let bird_right = bird.x + 1;
    let bird_top = bird.y;
    let bird_bottom = bird.y + 1;

    let obstacle_left = self.x;
    let obstacle_right = self.x + self.width;
    let obstacle_top = self.y;
    let obstacle_bottom = self.y + 1;

    // SAT (Separating Axis Theorem) simplified
    bird_right > obstacle_left &&
    bird_left < obstacle_right &&
    bird_bottom > obstacle_top &&
    bird_top < obstacle_bottom
}
```

**Complexity**: O(n) where n = number of obstacles
**Optimization opportunity**: Could use spatial partitioning for thousands of obstacles

## Error Handling

Uses `color_eyre` for rich error messages:

```rust
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;  // Install panic hook
    ratatui::run(app)?;      // Run game
    Ok(())
}
```

**Benefits**:
- Pretty error formatting
- Stack traces with context
- Panic backtraces on crash

## Dependencies Analysis

### ratatui (0.30.0)
- **Purpose**: Terminal UI framework
- **Usage**: 
  - `Frame`: Canvas for drawing
  - `Paragraph`: Text rendering
  - `Layout`: Arranging UI sections
  - `Style`: Colors and text formatting
- **Alternatives**: cursive, druid

### crossterm (0.29.0)
- **Purpose**: Terminal control and input
- **Usage**:
  - `event::{read, poll}`: Input polling
  - `KeyCode`: Key parsing
  - `Event::Key`: Key event handling
- **Alternatives**: termion, ncurses

### color_eyre (0.6.3)
- **Purpose**: Error handling and reporting
- **Usage**:
  - `Result<T>`: Error type
  - Pretty panic messages
- **Alternatives**: anyhow, eyre

## Performance Considerations

### Time Complexity
- **Input handling**: O(1)
- **Physics update**: O(1)
- **Obstacle update**: O(n) where n = active obstacles
- **Collision detection**: O(n)
- **Rendering**: O(GAME_WIDTH × GAME_HEIGHT)

### Space Complexity
- **Game state**: O(1) fixed structures
- **Obstacles**: O(n) where n = active obstacles (typically 2-4)
- **Render buffer**: O(GAME_WIDTH × GAME_HEIGHT) = O(1600)

### Optimizations Applied
- ✅ Frame rate limiting (poll with timeout)
- ✅ Obstacle removal (off-screen cull)
- ✅ Single-pass collision detection
- ✅ Buffer reuse (allocated once per frame)

### Potential Optimizations
- [ ] Spatial grid for collision detection
- [ ] Object pool for obstacles
- [ ] Unicode character caching
- [ ] Dirty rectangle rendering
- [ ] Multi-threaded input polling

## Testing Strategy

### Unit Testing
Could be added for:
- Bird physics (jump velocity, gravity)
- Collision detection (AABB)
- Obstacle spawning patterns
- Game state transitions

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bird_jump() {
        let mut bird = Bird::new();
        bird.jump();
        assert_eq!(bird.jumps_left, 2);
        assert_eq!(bird.velocity, -4.0);
    }

    #[test]
    fn test_collision_detection() {
        let bird = Bird { x: 5, y: 10, ..Default::new() };
        let obstacle = Obstacle::new(5, 10, ObstacleType::Tree);
        assert!(obstacle.collides_with_bird(&bird));
    }
}
```

### Integration Testing
Manual testing checklist:
- [ ] Bird jumps respond to space key
- [ ] Gravity pulls bird down
- [ ] Obstacles spawn at correct rate
- [ ] Obstacles move at correct speed
- [ ] Collision detection triggers game over
- [ ] Score increments
- [ ] Game over screen displays final score
- [ ] Q key exits game

### Load Testing
- Game runs at 10 FPS consistently
- Memory usage stable over long sessions
- No visual artifacts or flicker

## Extensibility

### Adding New Features

**New Obstacle Type:**
```rust
#[derive(Clone, Debug)]
enum ObstacleType {
    Tree,
    Rock,
    Cloud,
    // NEW:
    Spike,  // Ground hazard
}

// In new() method:
Spike => {
    width = 1;
    symbol = '▲';
    y = GAME_HEIGHT - 1;
}
```

**Power-ups:**
```rust
#[derive(Clone)]
struct PowerUp {
    x: u16,
    y: u16,
    power_type: PowerUpType,
}

enum PowerUpType {
    ExtraJump,
    Shield,
    SlowTime,
}
```

**Difficulty Levels:**
```rust
struct DifficultyConfig {
    gravity: f32,
    spawn_rate: usize,
    jump_power: i16,
    obstacle_speed: u16,
}

impl DifficultyConfig {
    fn easy() -> Self { /* ... */ }
    fn normal() -> Self { /* ... */ }
    fn hard() -> Self { /* ... */ }
}
```

### Refactoring Suggestions

1. **Separate game logic from rendering**:
   - Create `game_logic.rs` module
   - Keep rendering in `render.rs`

2. **Configuration system**:
   - Move constants to config struct
   - Load from YAML or TOML file

3. **Entity trait**:
   ```rust
   trait Entity {
       fn update(&mut self);
       fn render(&self) -> char;
       fn get_bounds(&self) -> Rect;
   }
   ```

4. **Event system**:
   - Create event enum for jumps, collisions
   - Event handler pattern for extensibility

## Debugging

### Enable Logging
Add `tracing` crate:
```rust
use tracing::{debug, info, warn};

fn update() {
    info!("Frame: {}", self.frame_count);
    debug!("Bird at ({}, {})", self.bird.x, self.bird.y);
    for obstacle in &self.obstacles {
        debug!("Obstacle at ({}, {})", obstacle.x, obstacle.y);
    }
}
```

### Console Output
During development, print to stderr (won't interfere with TUI):
```rust
eprintln!("Debug: bird.y = {}", bird.y);
```

### Common Issues

**Bird falls through ground**
- Check: `if self.y >= GAME_HEIGHT - 1`
- Ensure clamping happens in `update()`

**Obstacles not spawning**
- Check: `SPAWN_RATE` value
- Verify: Frame counter increments
- Debug: Print `frame_count % SPAWN_RATE`

**Collision not detected**
- Verify: AABB logic
- Debug: Print bird and obstacle positions
- Check: Coordinate system (is Y down or up?)

---

## Building and Releasing

### Development Build
```bash
cargo build
./target/debug/jumping-bird
```

### Release Build
```bash
cargo build --release
./target/release/jumping-bird
```

### Optimizations in Cargo.toml
```toml
[profile.release]
codegen-units = 1      # Single-threaded compilation
lto = true             # Link-time optimization
opt-level = "s"        # Size-optimized
strip = true           # Strip symbols
```

Result: ~3MB executable, ~10 FPS consistent performance

---

**Happy coding! 🐤**
