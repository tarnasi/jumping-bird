# Jumping Bird - Gameplay Guide

## Objective

Navigate your bird through an endless stream of obstacles, jumping at the right moments to avoid collisions. The longer you survive, the higher your score!

## Core Mechanics

### The Jump System

Your bird has a unique jump mechanic that rewards skillful control:

1. **Landing**: When your bird touches the ground, you're granted **3 jumps**
2. **Mid-Air Jumps**: Use these 3 jumps while airborne to navigate obstacles
3. **Stacking Jumps**: You can use multiple jumps in quick succession
4. **Resetting**: All jumps reset the moment you land again

**Example Jump Sequence:**
```
Ground → Jump (2 left) → Jump (1 left) → Jump (0 left) → Fall → Land (3 restored)
```

### Physics

The game uses realistic physics:

- **Gravity**: Pulls the bird downward every frame
- **Air Resistance**: Minimal; velocity changes linearly with gravity
- **Jump Velocity**: Fixed upward velocity independent of timing
- **Maximum Height**: Determined by gravity and jump power

**Understanding Jump Arcs:**

```
Single Jump Arc:
      ╱╲
    ╱    ╲      ← Peak height
   ╱      ╲
 Ground    ↓ Returns to ground

Double Jump Arc:
      ╱╲  ╱╲
    ╱    ╲╱  ╲  ← Second peak higher than first
   ╱      ↓    ╲
 Ground         ↓
```

### Obstacle Types

Three types of obstacles challenge you:

#### 🌲 Tree
- **Location**: Ground level
- **Size**: Wide (3 characters)
- **Strategy**: Jump over with height to spare
- **Difficulty**: Requires significant jump timing
- **Common**: Frequent spawns

#### ◆ Rock
- **Location**: Ground level
- **Size**: Narrow (2 characters)
- **Strategy**: Jump over or potentially thread between
- **Difficulty**: Quick reaction time needed
- **Common**: Moderately frequent

#### ☁ Cloud
- **Location**: Mid-air (altitude ~3)
- **Size**: Very wide (4 characters)
- **Strategy**: Duck under by staying low OR jump over from high
- **Difficulty**: Requires vertical space awareness
- **Common**: Less frequent, more dangerous

## Advanced Techniques

### Efficient Jumping

**Minimal Jumps Strategy**: Use fewest jumps possible to preserve control
```
Good approach: Jump once to clear obstacle, land safely
Bad approach: Jump three times right away, lose air control
```

### Timing and Spacing

Obstacles spawn every 15 frames (~1.5 seconds). This gives you time to:

1. Observe approaching obstacle
2. Jump to clear it
3. Land before next obstacle arrives

**Safe landing window**: You usually have 0.5+ seconds between obstacles

### Pattern Recognition

Over time, you'll notice patterns:

- Trees appear frequently
- Rocks follow most trees
- Clouds appear less often
- Obstacle timing is predictable

Exploit this by:
- Planning jumps in advance
- Positioning yourself for the next obstacle while clearing current one
- Jumping earlier for faster obstacles

### Risk vs. Reward

Every jump decision has consequences:

| Decision | Benefit | Risk |
|----------|---------|------|
| Jump early | More height, safety margin | Uses jump before fully needed |
| Jump late | Conserve jumps, land sooner | Collision risk, less height |
| Use multiple jumps | Maximum control | Vulnerable after last jump |
| Save jumps | Keep options open | Risk from insufficient height |

## Score Progression

### How Scoring Works

- **1 point per frame** (10 FPS game rate)
- **Displayed as**: Score / 10 = survival time in seconds
- **Score 100** = 10 seconds survived
- **Score 500** = 50 seconds survived

### Milestones

| Time | Score | Difficulty | Notes |
|------|-------|-----------|-------|
| 0-10s | 0-100 | Tutorial | Learning controls |
| 10-30s | 100-300 | Beginner | Establishing rhythm |
| 30-60s | 300-600 | Intermediate | Pattern mastery |
| 60-120s | 600-1200 | Advanced | High precision needed |
| 120s+ | 1200+ | Expert | Rarely achieved |

## Common Mistakes

### ❌ Mistake 1: Jumping Too Early
**Problem**: Jumping at obstacles from far away
**Result**: Loss of height by the time you reach obstacle
**Solution**: Jump closer to obstacles, use momentum

### ❌ Mistake 2: Wasting Jumps
**Problem**: Using all 3 jumps when 1 would suffice
**Result**: Defenseless when next obstacle arrives
**Solution**: Use minimum jumps needed to clear current obstacle

### ❌ Mistake 3: Panic Jumping
**Problem**: Spamming jump button at obstacles
**Result**: Unpredictable, uncontrolled movement
**Solution**: Calm, deliberate jump timing

### ❌ Mistake 4: Ignoring Clouds
**Problem**: Treating clouds like ground obstacles
**Result**: Collision while trying to jump over
**Solution**: Either jump over from ground, or duck under from high position

### ❌ Mistake 5: Poor Landing Spot
**Problem**: Landing directly in path of next obstacle
**Result**: Immediate collision after landing
**Solution**: Land behind obstacles, jump over next

## Best Practices

### ✅ Best Practice 1: Smooth, Rhythmic Jumping
- Find a comfortable jump rhythm
- Anticipate obstacles 2-3 steps ahead
- Flow with the game's pace

### ✅ Best Practice 2: Preserve Jumps
- Land before using all 3 jumps when possible
- Only go aggressive when obstacles demand it
- Treat the 3rd jump as insurance

### ✅ Best Practice 3: Vertical Positioning
- Maintain medium height for flexibility
- Too low: Can't jump over tall obstacles
- Too high: No room to dodge clouds

### ✅ Best Practice 4: Learn Patterns
- Notice obstacle sequences (e.g., Tree → Rock → Cloud)
- Predict landing positions for obstacles
- Plan 2-3 jumps ahead

### ✅ Best Practice 5: Stay Calm Under Pressure
- Don't panic when obstacles pile up
- Trust your muscle memory
- Take breaks if getting frustrated

## Learning Progression

### Stage 1: Learning (First Game)
**Goals**: 
- Get comfortable with jump mechanics
- Understand how gravity works
- Survive 10 seconds

**Focus**: Let obstacles come to you, simple single jumps

### Stage 2: Adaptation (Games 2-5)
**Goals**:
- Recognize obstacle types
- Time jumps more precisely
- Survive 30 seconds

**Focus**: Different jump heights for different obstacles

### Stage 3: Optimization (Games 6-10)
**Goals**:
- Anticipate spawning patterns
- Use 2-jump sequences efficiently
- Survive 60 seconds

**Focus**: Minimizing jump usage, flow state

### Stage 4: Mastery (Games 11+)
**Goals**:
- Achieve 2+ minute runs
- Handle rapid obstacles
- Consistent high scores

**Focus**: Perfect timing, reading ahead, peak performance

## Session Tips

### Before Playing
- Ensure your terminal is at least 90x25 characters
- Sit in comfortable position for keyboard access
- Clear your mind and focus

### During Playing
- Keep eyes on bird position, not just obstacles
- Feel the rhythm of spawning
- Notice when you make mistakes

### After Playing
- Review high-score runs
- Practice specific difficult sequences
- Take breaks (20 min play, 5 min rest)

## Difficulty Adjustments

If the game is too easy:
- Increase `SPAWN_RATE` for more obstacles
- Increase `GRAVITY` for faster falling
- Decrease `JUMP_POWER` for shorter jumps

If the game is too hard:
- Decrease `SPAWN_RATE` for fewer obstacles
- Decrease `GRAVITY` for slower falling
- Increase `JUMP_POWER` for longer jumps

See [DEVELOPMENT.md](DEVELOPMENT.md) for implementation details.

## World Records

Challenge yourself to beat these time milestones:

| Milestone | Time | Difficulty |
|-----------|------|-----------|
| 🥉 Bronze | 60 seconds | Advanced |
| 🥈 Silver | 120 seconds | Expert |
| 🥇 Gold | 180 seconds | Master |
| 💎 Platinum | 300+ seconds | Legendary |

Keep track of your personal best!

---

**Remember**: Every expert bird was once a beginner! Keep practicing and have fun! 🐤
