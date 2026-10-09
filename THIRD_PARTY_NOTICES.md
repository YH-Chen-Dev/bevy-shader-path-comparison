# Third-Party Notices

This repository contains shader implementations adapted from upstream Bevy examples. The repository's [LICENSE](LICENSE) covers the author's original contributions; it does not replace the licenses applicable to upstream source code or external dependencies.

## Bevy shader example source

- **Upstream project:** [Bevy](https://github.com/bevyengine/bevy)
- **Reference release:** [`release-0.10.0`](https://github.com/bevyengine/bevy/tree/release-0.10.0). This identifies the referenced upstream examples, not a verified original capstone dependency commit.
- **Case 1 upstream source:** [`assets/shaders/animate_shader.wgsl`](https://github.com/bevyengine/bevy/blob/release-0.10.0/assets/shaders/animate_shader.wgsl)
- **Case 2 upstream reference:** [`assets/shaders/instancing.wgsl`](https://github.com/bevyengine/bevy/blob/release-0.10.0/assets/shaders/instancing.wgsl)
- **Locally adapted shader files:**
  - [`wgsl-material-path/assets/shaders/case1/case1-dynamic-wgsl.wgsl`](wgsl-material-path/assets/shaders/case1/case1-dynamic-wgsl.wgsl)
  - [`rust-gpu-plugin-path/rust-gpu/crates/shader/src/case1-dynamic-rust.rs`](rust-gpu-plugin-path/rust-gpu/crates/shader/src/case1-dynamic-rust.rs)
  - [`wgsl-material-path/assets/shaders/case2/case2-gradient-wgsl.wgsl`](wgsl-material-path/assets/shaders/case2/case2-gradient-wgsl.wgsl)
  - [`rust-gpu-plugin-path/rust-gpu/crates/shader/src/case2-gradient-rust.rs`](rust-gpu-plugin-path/rust-gpu/crates/shader/src/case2-gradient-rust.rs)

Case 1 adapts Bevy's Oklab color-mixing WGSL example, including corresponding Rust-GPU code. Case 2 uses a Bevy shader example as a reference and implements a different world-position color output. For Bevy shader source reused here, this repository elects the **MIT** option from Bevy's `MIT OR Apache-2.0` source-code licenses. Bevy warns that some example assets have separate licenses; the two referenced shader source files have no per-file license header and are not assigned a separate license in Bevy 0.10's [CREDITS.md](https://github.com/bevyengine/bevy/blob/release-0.10.0/CREDITS.md).

The MIT license text from [Bevy `release-0.10.0` `LICENSE-MIT`](https://github.com/bevyengine/bevy/blob/release-0.10.0/LICENSE-MIT) is reproduced below. It intentionally has no named copyright holder line, matching the upstream license file.

```text
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Other external dependencies

The Bevy engine, [`bevy-rust-gpu`](https://github.com/Bevy-Rust-GPU/bevy-rust-gpu), Rust-GPU ecosystem crates, and other Cargo dependencies are fetched as external dependencies and are not relicensed by this repository. Dependency versions are tracked in the Cargo manifests and lockfiles. [`rust-gpu-builder`](https://github.com/Bevy-Rust-GPU/rust-gpu-builder) is referenced as a Git submodule, and its own upstream license applies to its separately obtained source. This notice does not replace the license notices supplied with those dependencies.
