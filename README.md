![Build Workflow](https://git.grzanka.org/radoslawg/Oxidized/actions/workflows/build.yaml/badge.svg)
<a href="https://gitmoji.dev">
  <img
    src="https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square"
    alt="Gitmoji"
  />
</a>

# Oxidized

Simple cyberpunk roguelike game writen in Rust and Raylib.
 - Graphics style is 2D Pixelart in rust color palette.
 - Sprites have size 32x32.
 - Skyscraper is made of levels connected by single staircase and/or elevator.
 - Skyscraper level consist of rooms with entrance from main, central corridor and also connected between them.
 - There are 10 levels in total, higher in the skyscraper the harder enemies are spawned and more complex level is generated.
 - At the end of 10th level is staircase to gang boss room.
 - Player and enemies should have graphics facing 8 ways.
 - It should be modelled with custom Entity Component System written from scratch
 - It should use logging using `log` and `simplelog` for logging
 - It should use unit tests to test functionality and strive for largest coverage possible.
  - As this is idiomatic to Rust, tests should be created in a same file as tested unit.
 - It should have gitea actions that would check if project builds, run all the tests and check coverage of those tests.
 - Code should be easy to follow and commented where necessary

## Architecture

```
src/
├── main.rs              # Entry point, window setup, main loop
├── lib.rs               # Public API for integration tests
├── game/
│   ├── mod.rs
│   └── state.rs         # Game state machine (Menu, Playing, Paused, GameOver)
├── ecs/
│   ├── mod.rs
│   ├── world.rs         # Entity storage, component management
│   ├── entity.rs        # Entity ID type
│   ├── component.rs     # Component trait + storage
│   └── system.rs        # System trait
├── components/          # Game-specific components
│   ├── mod.rs
│   ├── transform.rs     # Position, facing direction
│   ├── sprite.rs        # Sprite reference, animation state
│   ├── stats.rs         # STR, ACC, HP, Armor
│   └── actor.rs         # Player/Enemy marker, AI state
├── systems/             # Game-specific systems
│   ├── mod.rs
│   ├── input.rs         # Player input -> intent
│   ├── movement.rs      # Process movement
│   ├── combat.rs        # Bump-to-attack resolution
│   ├── render.rs        # Draw all entities
│   └── ai.rs            # Enemy behavior
├── level/
│   ├── mod.rs
│   ├── generator.rs     # Procedural level generation
│   └── tile.rs          # Tile types, map data
├── resources/           # Non-ECS shared state
│   ├── mod.rs
│   ├── atlas.rs         # Sprite atlas
│   └── timer.rs         # Timer with callbacks
└── utils/
    └── mod.rs           # Helpers, constants
```

## Mechanics
 - Movement should follow Vim navigation keys
 - Running into another actor attacks them
 - Player has attributes
    - Strength (STR): Attack Strength
    - Accuracy (ACC): Used for projectile weapons
    - Health Points (HP): How many damage can be taken before dead
    - Armor Points (A): How many damage points is soaked before HPs are deducted.

## Build, Test and coverage

### Repository
 - Commits should follow [gimoji](https://gitmoji.dev/) scheme of naming convention.

### Windows
On Windows, you have to install clang from visual studio (or at least download `libclang.dll`) and set `LIBCLANG_PATH` environment that would point to it.

### Build

```bash
cargo build
```

### Test

Simillarly to aboce use `make` to run tests.
```bash
cargo test
```
### Coverage
Coverage is performed using `tarpaulin`
```bash
cargo tarpaulin --out Html
```
In docker environment (i.e. during CI):
```bash
cargo tarpaulin --out Html --engine llvm
```

## Todo

### Core Systems
 - [ ] Implement game state machine (menu, gameplay, pause etc.)
 - [ ] Implement Entity Component System
 - [x] Set up logging with `log` and `simplelog`
 - [ ] Implement level generation algorithm (rooms, corridors, connections)
 - [ ] Implement enemy spawning system with difficulty scaling per floor
 - [ ] Implement timer with callback method/closure

### Gameplay
 - [ ] Implement Vim-style movement (h/j/k/l + diagonals)
 - [ ] Implement player controls
 - [ ] Implement melee combat (bump-to-attack)
 - [ ] Implement player attributes (STR, ACC, HP, Armor)
 - [ ] Implement staircase/elevator traversal between floors
 - [ ] Implement boss encounter on floor 10

### Graphics
 - [ ] Implement auto sprite atlas builder
 - [ ] Provide player sprites (8 directional)
 - [ ] Provide enemy sprites (8 directional)
 - [ ] Create tileset for rooms/corridors
 - [ ] Implement sprite direction system (8-way facing)
 - [ ] Implement glitch shader for main menu screen - https://www.youtube.com/watch?v=RTwPxsvLN_8

### Polish
 - [ ] Add sound effects / music
 - [ ] Implement save/load system
 - [ ] Add UI for player stats display

### Development
 - [ ] Read more on tarpaulin
