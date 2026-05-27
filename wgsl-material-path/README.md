# WGSL Material Path
This directory contains the WGSL-based Bevy material examples used in the shader path comparison project. Each example registers a custom Bevy material and loads its shader in [`assets/shaders/`](./assets/shaders/).

## Cases
| Case | Example | Shader |
|---|---|---|
| Case 1 | [`case1-wgsl-app.rs`](./apps/case1-wgsl-app.rs) | [`case1-dynamic-wgsl.wgsl`](./assets/shaders/case1/case1-dynamic-wgsl.wgsl) |
| Case 2 | [`case2-wgsl-app.rs`](./apps/case2-wgsl-app.rs) | [`case2-gradient-wgsl.wgsl`](./assets/shaders/case2/case2-gradient-wgsl.wgsl) |

## Build
From this directory:
```bash
cargo check --examples
```

For optimized builds:
```bash
cargo build --examples --release
```

## Run
From this directory:
```bash
cargo run --example case1-wgsl-app
```

```bash
cargo run --example case2-wgsl-app
```

> The examples currently close automatically after a short delay to support command-line reproduction checks.

## Notes
- Shader paths are resolved through Bevy's asset system.
- Runtime verification for this directory focuses on whether the examples build, launch, load their WGSL shaders, and render the corresponding cube demo scene.
