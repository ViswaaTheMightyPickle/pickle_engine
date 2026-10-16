# Pick(le)Engine

**Pick the engine. Pickle the world.**

Pick(le)Engine is an experimental 2D game engine written in Rust. The name is a small pun: it is both an engine you can *pick* for a project and, quite literally, a *pickle engine*.

The goal is to combine two kinds of simulation in one editor and one executable:

- Chunk-based cellular materials such as sand and powder.
- Rigid-body objects built from sprites, material masks, and collision masks.

The first milestone is deliberately small: place an imported object in a scene, let it fall onto terrain, and run a granular material beside it as particles travel across chunk boundaries.

> [!IMPORTANT]
> Pick(le)Engine is currently a foundation, not a finished or playable engine. The shared interfaces compile and the workspace is tested, but most simulation and editor features are still being implemented.

## What works today

- A four-crate Rust workspace with clean dependency boundaries.
- Shared engine types for cells, chunks, materials, objects, transforms, commands, and view snapshots.
- Sparse `32 x 32` world chunks stored in a `BTreeMap`.
- Correct chunk addressing for positive and negative world coordinates.
- Reading and writing cells across chunk boundaries.
- A read-only snapshot of visible world cells for rendering.
- Stable object IDs and placeholder physics state storage.
- A native eframe window with Play/Pause, Reset, and a test button that places one sand cell.
- Pinned Rapier, eframe, and image dependencies with a reproducible `Cargo.lock`.
- Passing workspace builds, tests, formatting, and Clippy checks.

## Current status

| Area | Status | What that means |
| --- | --- | --- |
| Core world | Foundation ready | Chunk storage, coordinates, cell access, and snapshots exist. Sand movement is next. |
| Physics | Interface only | Objects receive stable IDs, but Rapier bodies, gravity, terrain, and colliders are not wired up yet. |
| Asset import | Interface only | The importer API exists, but PNG decoding and mask validation are still TODO. |
| Editor | First window | The app opens and exposes basic controls, but there is no scene viewport or asset-placement workflow yet. |
| Integration | Not started | The crates compile together; the playable demonstration does not exist yet. |

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

## Simulation rules

- **Coordinates:** +X points right and +Y points down.
- **Chunks:** each chunk contains `32 x 32` cells.
- **Storage:** cell arrays are row-major.
- **Scale:** 16 simulation cells equal one metre in the physics world.
- **Timestep:** simulation advances at a fixed 60 Hz.
- **Ownership:** rigid-object material belongs to its object, not the powder grid.
- **Mutation:** the editor queues commands; simulation systems apply them at step boundaries.

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

- [ ] Move sand downward and diagonally without duplicating cells.
- [ ] Prove sand can cross positive and negative chunk boundaries.
- [ ] Decode sprite, action-mask, and collision-mask PNGs.
- [ ] Reject mismatched dimensions, unknown materials, partial alpha, and invalid collision colors.
- [ ] Spawn a Rapier rigid body with a simple collider.
- [ ] Add terrain and verify that a falling body comes to rest.
- [ ] Render world cells and physics snapshots in the editor.
- [ ] Queue placement and reset commands through the shared contract.
- [ ] Deliver one reproducible integrated demo.

## Why Rust?

Rust gives the project explicit ownership, strong boundaries between subsystems, predictable native performance, and a compiler that catches integration mistakes early. That matters for an engine where the editor, cellular world, asset pipeline, and physics simulation all need to agree on who owns what.

Pick(le)Engine is small on purpose right now. The aim is to build one dependable vertical slice first—and only then start opening the jar.
