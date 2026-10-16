# Hybrid Pixel Engine

A bare-bones Rust workspace for a hybrid pixel game engine MVP. The repository starts with the shared interfaces and TODO-marked subsystem placeholders from the coding handoff.

## Workspace

- `engine_core`: shared types, chunked world storage, and cell operations
- `engine_physics`: private Rapier adapter boundary
- `engine_assets`: PNG sprite and mask importer boundary
- `engine_app`: native eframe application and integration point

Dependencies flow toward `engine_core`; core does not depend on UI, physics, or image-decoder crates.

## Requirements

- Stable Rust with Edition 2024 support
- Linux development packages required by eframe/winit, or MSVC C++ build tools on Windows

## Commands

```sh
rustup update stable
cargo check --workspace
cargo test --workspace
cargo run -p engine_app
cargo fmt --all
cargo clippy --workspace --all-targets
```

## MVP boundaries

The initial code is a contract, not a completed engine. Powder movement, Rapier bodies and colliders, strict PNG-mask validation, and the editor viewport remain intentionally marked as TODOs.

Coordinates use +X right and +Y down. Chunks contain 32 x 32 cells, one simulation cell is 1/16 metre, and the fixed simulation step is 1/60 second.
