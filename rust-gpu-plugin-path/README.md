# Rust-GPU Plugin Path
This directory contains the Rust-GPU plugin-based path, which uses the existing [`bevy-rust-gpu`](https://github.com/Bevy-Rust-GPU/bevy-rust-gpu) plugin to integrate Rust-GPU shader artifacts into Bevy. The path is split into two parts:

- [`rust-gpu/`](./rust-gpu/) builds the Rust shader crate into a runtime shader artifact.
- [`bevy-app/`](./bevy-app/) runs the Bevy examples that load and apply the generated shader artifact.

## Cases
| Case | Bevy Example | Rust Shader Source |
|---|---|---|
| Case 1 | [`case1-rust-app.rs`](./bevy-app/apps/case1-rust-app.rs) | [`case1-dynamic-rust.rs`](./rust-gpu/crates/shader/src/case1-dynamic-rust.rs) |
| Case 2 | [`case2-rust-app.rs`](./bevy-app/apps/case2-rust-app.rs) | [`case2-gradient-rust.rs`](./rust-gpu/crates/shader/src/case2-gradient-rust.rs) |

## Build Shader Artifact
From [`rust-gpu/`](./rust-gpu/):
```bash
cargo run --release -- crates/shader ../bevy-app/assets/rust-gpu/shader.rust-gpu.msgpack
```

This generates the shader artifact used by the Bevy app:
```text
bevy-app/assets/rust-gpu/shader.rust-gpu.msgpack
```

## Run Bevy Examples
After generating the shader artifact, run the Bevy examples from [`bevy-app/`](./bevy-app/):
```bash
cargo run --example case1-rust-app
```

```bash
cargo run --example case2-rust-app
```

> The examples currently close automatically after a short delay to support command-line reproduction checks.

## Entry Points
The generated shader artifact is built from the shader crate that defines entry points for both cases:
```text
case1::vertex_main_case1
case1::fragment_main_case1
case2::vertex_main_case2
case2::fragment_main_case2
```

## Generated Files
The following files and directories are not committed:
```text
bevy-app/assets/rust-gpu/
bevy-app/generated/
rust-gpu/spirt-passes/
```

## Notes
- [`rust-gpu-builder`](./rust-gpu/crates/rust-gpu-builder/) is tracked as a submodule.
- [`rust-toolchain`](./rust-gpu/rust-toolchain) pins the toolchain used by the Rust-GPU shader workspace.
- Runtime verification for this directory focuses on whether the shader artifact builds, the Bevy examples launch, the generated shader artifact is applied through the configured entry points, and the corresponding cube demo scenes render.
