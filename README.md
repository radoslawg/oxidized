![Build Workflow](https://git.grzanka.org/radoslawg/Oxidized/actions/workflows/build.yaml/badge.svg)
<a href="https://gitmoji.dev">
  <img
    src="https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square"
    alt="Gitmoji"
  />
</a>

# Oxidized

Simple cyberpunk roguelike game writen in Rust and Raylib.
 - Graphics style is 3D Low Poly in [Apollo](https://lospec.com/palette-list/apollo) color palette.
 - Skyscraper is made of levels connected by single staircase and/or elevator.
 - Skyscraper level consist of rooms with entrance from main, central corridor and also connected between them.
 - There are 10 levels in total, higher in the skyscraper the harder enemies are spawned and more complex level is generated.
 - At the end of 10th level is staircase to gang boss room.
 - It should be modelled with custom Entity Component System written from scratch
 - It should use logging using `log` and `simplelog` for logging
 - It should use unit tests to test functionality where possible.
  - As this is idiomatic to Rust, tests should be created in a same file as tested unit.
 - It should have gitea actions that would check if project builds and run all the tests.
 - Code should be easy to follow and commented where necessary


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
### Build

```bash
cargo build
```

### Test

Simillarly to above use `make` to run tests.
```bash
cargo test
```

## Todo

### Core Systems
 - [ ] Implement game state machine (menu, gameplay, pause etc.)
 - [ ] Implement Entity Component System
 - [x] Set up logging with `log` and `simplelog`
 - [ ] Implement level generation algorithm based on simple ascii map definition.
 - [ ] Implement timer with callback method/closure

### Gameplay
 - [ ] Implement Vim-style movement (h/j/k/l + diagonals)
 - [ ] Implement player controls
 - [ ] Implement melee combat (bump-to-attack)
 - [ ] Implement player attributes (STR, ACC, HP, Armor)
 - [ ] Implement staircase/elevator traversal between floors
 - [ ] Implement boss encounter on floor 10

### Graphics
 - [ ] Create main character model with basic idle animation.
 - [ ] Create simple floor tile model.
 - [ ] Implement glitch shader for main menu screen - https://www.youtube.com/watch?v=RTwPxsvLN_8

### Polish
 - [ ] Add sound effects / music
 - [ ] Implement save/load system
 - [ ] Add UI for player stats display

### Future
 - [ ] Read more on tarpaulin and coverage
