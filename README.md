# Pick(le)Engine

**Pick the engine. Pickle the world.**

Pick(le)Engine is an experimental 2D game engine written in Rust. The name is a small pun: it is both an engine you can *pick* for a project and, quite literally, a *pickle engine*.

The goal is to combine two kinds of simulation in one editor and one executable:

- Chunk-based cellular materials such as sand and powder.
- Rigid-body objects built from sprites, material masks, and collision masks.

The first milestone is deliberately small: place an imported object in a scene, let it fall onto terrain, and run a granular material beside it as particles travel across chunk boundaries.

> [!IMPORTANT]
> Pick(le)Engine is currently a foundation, not a finished or playable engine. The workspace has tests, but the asset, physics, and editor workflows needed for the first playable milestone are still being implemented.

## What works today

- A four-crate Rust workspace with clean dependency boundaries.
- Shared engine types for cells, chunks, materials, objects, transforms, commands, and view snapshots.
- Sparse `32 x 32` world chunks stored in a `BTreeMap`.
- Correct chunk addressing for positive and negative world coordinates.
- Reading and writing cells across chunk boundaries.
- Sand movement down and diagonally, with active-chunk updates and tests for crossing a positive chunk boundary.
- A read-only snapshot of visible world cells and a basic viewport that renders sand and stone.
- A Rapier pipeline with a headless collision test, plus stable object IDs and placeholder physics state storage.
- A native eframe window with Play/Pause, Reset, and a test button that places one sand cell.
- Pinned Rapier, eframe, and image dependencies with a reproducible `Cargo.lock`.

## Current status

| Area | Status | What that means |
| --- | --- | --- |
| Core world | Basic sand simulation | Chunk storage, cell access, snapshots, and sand movement exist. Movement is currently restricted to an `8 x 8`-chunk area (cell coordinates `0..=255` on each axis). |
| Physics | Rapier pipeline scaffold | The pipeline steps in a headless collision test, but spawning imported bodies, syncing transforms, and connecting terrain remain unfinished. |
| Asset import | Interface only | The importer API exists, but PNG decoding and mask validation are still TODO. |
| Editor | Basic viewport | Sand and stone cells render; mouse-based placement and imported-object workflows are still missing. |
| Integration | Partial | The app steps the world and physics pipeline, but the first playable demonstration does not exist yet. |

In short: the skeleton is healthy, the organs are labelled, and now the creature needs to be brought to life.

## The first playable milestone

The initial target is one native application where you can:

1. Import a sprite with an action/material mask and a collision mask.
2. Place that object into a small scene.
3. Press Play and watch it fall onto simple terrain.
4. Run sand or powder on a chunked grid beside it.
5. Pause and reset the simulation reliably.

Pixel erasing, regenerated colliders, disconnected-piece splitting, liquids, gases, chemistry, save files, and visual scripting are later goals—not promises for the first slice.

## Project layout

```text
crates/
├── engine_core/      shared types and chunked cellular world
├── engine_physics/   Rapier adapter and rigid-body state
├── engine_assets/    sprite and mask import pipeline
└── engine_app/       native editor and simulation orchestration
```

Dependencies flow toward `engine_core`. The core crate stays independent of Rapier, eframe, and image-decoder types, while physics internals remain private to `engine_physics`.

## Simulation rules and targets

- **Coordinates:** +X points right and +Y points down.
- **Chunks:** each chunk contains `32 x 32` cells.
- **Storage:** cell arrays are row-major.
- **Scale:** 16 simulation cells equal one metre in the physics world.
- **Timestep:** the target is fixed 60 Hz. Rapier is configured with a 60 Hz timestep, but the app currently steps on each UI update rather than scheduling fixed-rate updates.
- **Ownership:** rigid-object material belongs to its object, not the powder grid.
- **Mutation:** the intended design is for the editor to queue commands for simulation step boundaries; the current app mutates the world directly.

## Getting started

Install the latest stable Rust toolchain from [rustup.rs](https://rustup.rs/), then run:

```sh
rustup update stable
cargo check --workspace
cargo test --workspace
cargo run -p engine_app
```

Before committing changes:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

On Fedora, eframe may require the following system development packages:

```sh
sudo dnf install gcc gcc-c++ clang-devel pkgconf-pkg-config \
  libxkbcommon-devel libxcb-devel openssl-devel
```

On Windows, install the MSVC C++ build tools if requested by `rustup`.

## Near-term roadmap

- [x] Implement downward and diagonal sand movement, with tests for conservation and positive chunk-boundary crossing.
- [ ] Decide whether sand should move at negative coordinates; current movement is restricted to an `8 x 8`-chunk area.
- [ ] Add mouse-based sand and stone placement in the viewport.
- [ ] Decode sprite, action-mask, and collision-mask PNGs.
- [ ] Reject mismatched dimensions, unknown materials, partial alpha, and invalid collision colors.
- [x] Step Rapier in a headless test with a body and floor collider.
- [ ] Spawn imported objects as Rapier bodies, build colliders, and sync their transforms.
- [ ] Connect world terrain to physics and verify that a falling object comes to rest.
- [x] Render world cells in the editor viewport.
- [ ] Render physics objects in the editor and schedule simulation at a fixed timestep.
- [ ] Queue placement and reset commands through the shared contract.
- [ ] Deliver one reproducible integrated demo.

## Why Rust?

Rust gives the project explicit ownership, strong boundaries between subsystems, predictable native performance, and a compiler that catches integration mistakes early. That matters for an engine where the editor, cellular world, asset pipeline, and physics simulation all need to agree on who owns what.

Pick(le)Engine is small on purpose right now. The aim is to build one dependable vertical slice first—and only then start opening the jar.
