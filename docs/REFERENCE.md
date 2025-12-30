# Jumping Bird - Code Reference

Complete API documentation and code examples for Jumping Bird.

## Module Structure

```
main.rs
├── Constants (Game Parameters)
├── Structs
│   ├── Bird
│   ├── Obstacle
│   ├── ObstacleType
│   └── Game
├── Functions
│   ├── render()
│   ├── render_game_area()
│   ├── app()
│   └── main()
└── Implementations
    ├── Bird::new()
    ├── Bird::jump()
    ├── Bird::update()
    ├── Obstacle::new()
    ├── Obstacle::update()
    ├── Obstacle::is_off_screen()
    ├── Obstacle::collides_with_bird()
    ├── Game::new()
    ├── Game::handle_input()
    └── Game::update()
```

## Constants Reference

### Dimensions

```rust
const GAME_WIDTH: u16 = 80;      // Playfield width
const GAME_HEIGHT: u16 = 20;     // Playfield height
```

**Impact**: Defines game area boundaries. Obstacles won't spawn beyond `GAME_WIDTH`, and bird can't go above y=0 or below y=GAME_HEIGHT-1.

**Typical Values**:
- Small (40×10): Fast-paced, mobile friendly
- Default (80×20): Balanced for most terminals
- Large (120×30): Slower-paced, more space

### Bird Parameters

```rust
const BIRD_START_X: u16 = 5;     // Initial bird X position
const BIRD_START_Y: u16 = 10;    // Initial bird Y position (middle)
const MAX_JUMPS: u8 = 3;         // Jumps per landing
const JUMP_POWER: i16 = 4;       // Jump velocity magnitude
const GRAVITY: f32 = 0.2;        // Gravity acceleration
```

**Bird Start Position**:
- X=5: Left side of screen (safe from spawning obstacles)
- Y=10: Middle height (room to jump up or down)

**Jump Physics**:
```
Single jump arc with JUMP_POWER=4, GRAVITY=0.2:
- Frame 0: y=10, v=0
- Jump: v=-4
- Frame 1: v=-3.8, y=6.2
- Frame 2: v=-3.6, y=2.6
- Peak: y≈-1 (clamped to 0)
- Total air time: ~8 frames
```

**Gravity Impact**:
- 0.1: Very floaty (easy)
- 0.2: Balanced (default)
- 0.3: Heavy (hard)

### Obstacle Parameters

```rust
const OBSTACLE_SPEED: u16 = 1;   // Pixels per frame
const SPAWN_RATE: usize = 15;    // Frames between spawns
```

**Obstacle Speed**:
- 1 px/frame: ~3 seconds to cross screen
- 2 px/frame: ~1.5 seconds (harder)
- 0.5 px/frame: ~6 seconds (easier)

**Spawn Rate**:
- 10: High obstacle density (very hard)
- 15: Default (balanced)
- 20: Low obstacle density (easy)
- 30: Very sparse (tutorial-like)

---

## Type Reference

### Bird Struct

```rust
#[derive(Clone, Copy, Debug)]
struct Bird {
    x: u16,              // Horizontal position (0 to 79)
    y: u16,              // Vertical position (0 to 19)
    velocity: f32,       // Current vertical velocity (pixels/frame)
    jumps_left: u8,      // Remaining jumps (0 to 3)
    is_jumping: bool,    // Currently airborne
}
```

**Field Ranges**:
| Field | Min | Max | Typical |
|-------|-----|-----|---------|
| x | 0 | 79 | 5 (fixed) |
| y | 0 | 19 | 10 (varies) |
| velocity | -4.0 | 4.0 | 0.0 (ground) |
| jumps_left | 0 | 3 | 3 (after landing) |

### Obstacle Struct

```rust
#[derive(Clone, Debug)]
struct Obstacle {
    x: u16,                      // Horizontal position
    y: u16,                      // Vertical position
    width: u16,                  // Collision width
    obstacle_type: ObstacleType, // Visual type
}

#[derive(Clone, Debug, PartialEq)]
enum ObstacleType {
    Tree,    // Ground, width=3, symbol=🌲
    Rock,    // Ground, width=2, symbol=◆
    Cloud,   // Air, width=4, symbol=☁
}
```

**Obstacle Properties**:
| Type | Y Position | Width | Speed | Symbol |
|------|-----------|-------|-------|--------|
| Tree | 19 | 3 | 1 | 🌲 |
| Rock | 19 | 2 | 1 | ◆ |
| Cloud | 3 | 4 | 1 | ☁ |

### Game Struct

```rust
#[derive(Debug)]
struct Game {
    bird: Bird,                  // Player character
    obstacles: Vec<Obstacle>,    // Active obstacles
    score: u32,                  // Current score (frames)
    game_over: bool,             // Game state
    frame_count: usize,          // Total frames elapsed
}
```

**Field Descriptions**:
- `bird`: Current bird state (position, velocity, jumps)
- `obstacles`: Vector of active obstacles (typically 2-6)
- `score`: Incremented each frame, displayed as score/10
- `game_over`: True when collision detected
- `frame_count`: Used for obstacle spawning patterns

---

## Function Reference

### Bird Methods

#### `Bird::new() -> Self`

Creates a new bird at starting position with full jumps.

```rust
let bird = Bird::new();
assert_eq!(bird.x, 5);
assert_eq!(bird.y, 10);
assert_eq!(bird.jumps_left, 3);
```

**Returns**: Initialized Bird instance
**Side Effects**: None

#### `Bird::jump(&mut self)`

Executes a jump if jumps remain available.

```rust
let mut bird = Bird::new();
bird.jump();  // jumps_left: 3 → 2
bird.jump();  // jumps_left: 2 → 1
bird.jump();  // jumps_left: 1 → 0
bird.jump();  // No effect, jumps_left: 0
```

**Precondition**: `self.jumps_left > 0`
**Side Effects**:
- Decrements `jumps_left`
- Sets `velocity = -4.0`
- Sets `is_jumping = true`

**Returns**: None

#### `Bird::update(&mut self)`

Updates bird position with physics simulation.

```rust
let mut bird = Bird::new();
bird.jump();
for _ in 0..10 {
    bird.update();  // Falls due to gravity
}
// Bird now lower than starting position
```

**Physics Steps**:
1. `velocity += GRAVITY` (accelerate down)
2. `y += velocity as u16` (apply velocity)
3. Clamp Y to bounds [0, GAME_HEIGHT-1]
4. Reset velocity if at boundaries
5. Restore jumps if on ground

**Returns**: None

### Obstacle Methods

#### `Obstacle::new(x: u16, y: u16, obstacle_type: ObstacleType) -> Self`

Creates a new obstacle with width determined by type.

```rust
let tree = Obstacle::new(80, 19, ObstacleType::Tree);
assert_eq!(tree.width, 3);

let cloud = Obstacle::new(80, 3, ObstacleType::Cloud);
assert_eq!(cloud.width, 4);
```

**Parameters**:
- `x`: Starting position (usually `GAME_WIDTH`)
- `y`: Vertical position
- `obstacle_type`: Type determines width and visual

**Returns**: Initialized Obstacle

#### `Obstacle::update(&mut self)`

Moves obstacle left one position per frame.

```rust
let mut obstacle = Obstacle::new(80, 19, ObstacleType::Tree);
obstacle.update();
assert_eq!(obstacle.x, 79);
obstacle.update();
assert_eq!(obstacle.x, 78);
```

**Side Effects**: Decrements x by `OBSTACLE_SPEED` (usually 1)

**Returns**: None

#### `Obstacle::is_off_screen(&self) -> bool`

Checks if obstacle has exited the left side of screen.

```rust
let mut obstacle = Obstacle::new(2, 19, ObstacleType::Rock);
assert!(!obstacle.is_off_screen());
obstacle.x = 0;
assert!(obstacle.is_off_screen());  // width=2, so 0+2 < 1
```

**Returns**: `true` if `x + width < 1`, else `false`

**Usage**: Used to remove off-screen obstacles from Vec

#### `Obstacle::collides_with_bird(&self, bird: &Bird) -> bool`

AABB collision detection between obstacle and bird.

```rust
let bird = Bird { x: 5, y: 19, ..Bird::new() };
let obstacle = Obstacle::new(5, 19, ObstacleType::Tree);
assert!(obstacle.collides_with_bird(&bird));  // Same position = collision

let bird_far = Bird { x: 20, y: 19, ..Bird::new() };
assert!(!obstacle.collides_with_bird(&bird_far));  // Distance = no collision
```

**Collision Formula**:
```rust
bird.right > obstacle.left &&      // Bird right edge past obstacle left
bird.left < obstacle.right &&      // Bird left edge before obstacle right
bird.bottom > obstacle.top &&      // Bird bottom past obstacle top
bird.top < obstacle.bottom         // Bird top before obstacle bottom
```

**Returns**: `true` if collision detected, `false` otherwise

### Game Methods

#### `Game::new() -> Self`

Creates a new game state.

```rust
let game = Game::new();
assert_eq!(game.score, 0);
assert!(!game.game_over);
assert_eq!(game.obstacles.len(), 0);
```

**Returns**: Initialized Game state
**Side Effects**: None

#### `Game::handle_input(&mut self, code: KeyCode)`

Processes keyboard input.

```rust
let mut game = Game::new();
game.handle_input(KeyCode::Char(' '));  // Jump
game.handle_input(KeyCode::Char('q'));  // Quit
game.handle_input(KeyCode::Char('x'));  // Ignored
```

**Key Mappings**:
- Space: `bird.jump()`
- Q or Esc: Set `game_over = true`
- Other keys: Ignored

**Returns**: None

#### `Game::update(&mut self)`

Single frame game logic update.

```rust
let mut game = Game::new();
game.update();  // Physics update
assert_eq!(game.frame_count, 1);
assert_eq!(game.score, 1);
```

**Update Sequence**:
1. `self.bird.update()` - Physics
2. Update all obstacles: `obstacle.update()`
3. Remove off-screen obstacles
4. Spawn new obstacle if `frame_count % SPAWN_RATE == 0`
5. Check collisions with all obstacles
6. Increment score
7. Increment frame_count

**Returns**: None

### Rendering Functions

#### `render(frame: &mut Frame, game: &Game)`

Main rendering function for terminal UI.

```rust
fn app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let game = Game::new();
    terminal.draw(|frame| render(frame, &game))?;
    Ok(())
}
```

**Layout**:
```
[Title + Score]
[   Game Area   ]
[Instructions  ]
```

**Renders**:
- Title bar with game name, score, jump count
- Game area with borders (game_area)
- Instruction/status footer

#### `render_game_area(frame: &mut Frame, game: &Game, game_area: Rect)`

Renders game entities within the game area.

**Process**:
1. Create 2D character buffer
2. Draw ground line (═)
3. Draw bird (🐤)
4. Draw obstacles (🌲, ◆, ☁)
5. Convert buffer to Lines
6. Render with Paragraph widget

**Complexity**: O(GAME_WIDTH × GAME_HEIGHT) = O(1600)

---

## Game Flow Diagram

```
main()
  └─ app()
      └─ Loop:
          ├─ terminal.draw(render)
          │   └─ render()
          │       ├─ Title + Score
          │       ├─ render_game_area()
          │       │   ├─ Ground
          │       │   ├─ Bird
          │       │   └─ Obstacles
          │       └─ Instructions
          ├─ poll(timeout)
          │   └─ event::read()
          │       └─ handle_input()
          │           ├─ Space → bird.jump()
          │           └─ Q → game_over = true
          └─ tick (100ms):
              └─ game.update()
                  ├─ bird.update()
                  ├─ obstacles.update()
                  ├─ spawn_obstacles()
                  └─ check_collisions()
```

---

## State Machines

### Bird State Machine

```
Landed (jumps_left=3)
    ↓ (Space pressed)
Jumping (jumps_left=2, velocity=-4)
    ↓ (physics)
Airborne (jumps_left=2, velocity increases)
    ↓ (Space pressed)
Jumping (jumps_left=1, velocity=-4)
    ↓ (physics)
Airborne (jumps_left=1, velocity increases)
    ↓ (physics)
Landed (jumps_left=3, y=19, velocity=0)
```

### Game State Machine

```
Playing
    ├─ (render frame)
    ├─ (handle input)
    ├─ (game.update())
    └─ (collision?)
        ├─ Yes → Game Over
        │        └─ (wait for Q)
        │           └─ Exit
        └─ No → Continue Playing
```

---

## Common Patterns

### Creating a Custom Obstacle Type

Add to `ObstacleType` enum and handle in spawn logic:

```rust
#[derive(Clone, Debug, PartialEq)]
enum ObstacleType {
    Tree,
    Rock,
    Cloud,
    Spike,  // NEW
}

// In Obstacle::new()
match obstacle_type {
    // ... existing types ...
    Spike => {
        width = 1;
        y = GAME_HEIGHT - 1;
    }
}

// In Game::update() spawning logic
let obstacle_type = match self.frame_count / SPAWN_RATE % 4 {
    0 => ObstacleType::Tree,
    1 => ObstacleType::Rock,
    2 => ObstacleType::Cloud,
    _ => ObstacleType::Spike,  // NEW
};
```

### Changing Difficulty on the Fly

```rust
// In Game struct, add difficulty field
struct Game {
    // ... existing fields ...
    difficulty_multiplier: f32,
}

// In update(), use multiplier
const SPAWN_RATE: usize = 15;
let adjusted_spawn = (SPAWN_RATE as f32 * self.difficulty_multiplier) as usize;
if self.frame_count % adjusted_spawn == 0 {
    // spawn obstacle
}
```

### Adding Score Multipliers

```rust
// Instead of score += 1
let score_increment = if self.bird.is_jumping {
    2  // Bonus points for staying airborne
} else {
    1
};
self.score += score_increment;
```

---

## Debugging Tips

### Printing Game State

Add to `Game::update()`:
```rust
eprintln!("Frame: {}", self.frame_count);
eprintln!("Bird: ({}, {})", self.bird.x, self.bird.y);
eprintln!("Velocity: {}", self.bird.velocity);
eprintln!("Obstacles: {}", self.obstacles.len());
eprintln!("Score: {}", self.score);
```

### Testing Jump Physics

```rust
#[test]
fn test_jump_arc() {
    let mut bird = Bird::new();
    let start_y = bird.y;
    bird.jump();
    
    let mut max_height = start_y;
    for _ in 0..20 {
        bird.update();
        max_height = min(max_height, bird.y);
    }
    
    assert!(max_height < start_y);  // Bird did go up
    assert!(bird.y >= start_y - 2);  // Returned near starting height
}
```

### Collision Debugging

```rust
for obstacle in &self.obstacles {
    if obstacle.collides_with_bird(&self.bird) {
        eprintln!("COLLISION!");
        eprintln!("Bird: ({}, {}) -> ({}, {})", 
            self.bird.x, self.bird.y, 
            self.bird.x + 1, self.bird.y + 1);
        eprintln!("Obstacle: ({}, {}) -> ({}, {})",
            obstacle.x, obstacle.y,
            obstacle.x + obstacle.width, obstacle.y + 1);
    }
}
```

---

## Performance Notes

### Complexity Analysis

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Bird update | O(1) | Single entity physics |
| Obstacle update | O(n) | n = active obstacles |
| Collision check | O(n) | n = active obstacles |
| Rendering | O(w×h) | w=80, h=20 → 1600 ops |
| Total per frame | O(n) | n typically 3-6 |

### Optimization Opportunities

```rust
// Could batch obstacle updates
for obstacle in &mut self.obstacles {
    obstacle.update();  // O(n) - already optimal
}

// Collision detection could use spatial partitioning
// But with max 6 obstacles, not needed
```

---

**This reference covers all public APIs and common patterns. For examples, see GAMEPLAY.md and DEVELOPMENT.md.** 🐤
