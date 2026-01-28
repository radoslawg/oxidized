![Build Workflow](https://git.grzanka.org/radoslawg/Oxidized/actions/workflows/build.yaml/badge.svg)

# Oxidized

Simple classic roguelike game writeen in Rust and Raylib.
 - Graphics style is fantasy, colorful 2D Pixelart.
 - Sprites have size 32x32.
 - Dungeon is made of levels connected by single staircase.
 - Dungeon level consist of rooms connected by corridors.
 - There are 10 levels in total, deeper the dungeon the harder enemies are spawned and larger dungeon is generated.
 - At the end of 10th level is staircase to boss lair.
 - Player and enemies should have graphics facing 8 ways.
 - It should be modelled with custom Entity Component System written from scratch
 - It should use logging using `log` and `simplelog` for logging
 - It should use unit tests to test functionality and strive for largest coverage possible.
 - It should have gitea actions that would check if project builds, run all the tests and check coverage of those tests.
 - Code should be easy to follow and commented where necessary

## Build

Because Windows version requires `libclang.dll` pointed by custom Environment Variable, simple `Makefile` is used to build the project.

```bash
make build
```

## Test

Simillarly to aboce use `make` to run tests.
```bash
make test
```
## Coverage
Coverage is performed using `tarpaulin`
```bash
make coverage
```
