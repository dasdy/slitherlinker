# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (uses a hardcoded example puzzle)
cargo run

# Run with a puzzle string
cargo run -- "10x10d0:b1a2a22a32a1b22b2b2a23b212a22d222a31b2c12c2d331d013e1a2c2122a1b2a3b13a02a"

# Run tests
cargo test

# Run a specific test module
cargo test parse::test
cargo test data::pattern::test
```

## Puzzle Input Format

`WxHdD:PUZZLE_CODE` where W=width, H=height, D=difficulty (optional). In the puzzle code, digits `0`–`3` are cell clues and lowercase letters encode runs of empty cells (`a`=skip 1, `b`=skip 2, ...).

## Architecture

The solver pipeline has four stages:

1. **Parse** (`parse.rs`) — Converts a puzzle string into a 2D grid of cell clue values (0–3 or -1 for unknown).

2. **Pattern deduction** (`patterns.rs`, `data/baked_in_patterns.rs`) — Applies 3×3 window patterns iteratively (up to 1000 passes) to derive forced edge states before invoking SAT. Patterns support rotation/reflection. New patterns go in `data/baked_in_patterns.rs`.

3. **SAT encoding** (`solve_common.rs`) — Encodes all cell-edge constraints and loop-continuity constraints as Boolean clauses via the `SlitherlinkerFormula` trait defined in `adapter.rs`. This abstraction supports two backends: **Varisat** (`solve_varisat.rs`, default) and **Splr** (`solve_splr.rs`).

4. **Output** (`data/solution.rs`) — Renders edge assignments as Unicode box-drawing ASCII art.

### Key data structures

- **`Puzzle`** (`data/puzzle.rs`) — Stores the cell grid and computes edge indices. Horizontal edges are indexed `[0, (1+xsize)×ysize)`, vertical edges follow. Key helpers: `edges_around_cell()`, `edges_around_edge()`, `edges_around_point()`.
- **`Edge`** enum: `Any | Unknown | Empty | Filled | EmptyStrict | OutOfBounds`
- **`Cell`** enum: `Zero | One | Two | Three | Nothing | Any | OutOfBounds`
- **`Pattern` / `PatternSolution`** (`data/pattern.rs`) — 3×3 windows used for deductive matching.

## Python detector (`../detector/`)

A separate component that converts puzzle screenshots to puzzle strings using a Keras/TensorFlow neural network.

```bash
cd ../detector
poetry run python detect.py --img <image_path> [--debug]
```
