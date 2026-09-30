# Contributing

Thanks for taking an interest in Bloom.

## Requirements

Rust 1.87 or newer, edition 2024. A GPU with WebGPU support is needed to run the
app; DX12 is the default backend on Windows.

## Building

The core build needs nothing beyond the Rust toolchain:

```sh
cargo build
```

Two features are optional and each links against a native library. Setup scripts
live in `scripts/`:

| Feature | Library | Setup |
| --- | --- | --- |
| `heif` | libheif | `./scripts/setup-heif.sh`, or `setup-heif.ps1` on Windows |
| `av` | FFmpeg | `./scripts/setup-av.sh`, or `setup-av.ps1` on Windows |

On macOS the scripts install the library with Homebrew. On Linux they check for
it and print the package to install. On Windows they vendor the dependency into
`vendor/` and set `VCPKG_ROOT` or `FFMPEG_DIR`, so open a new terminal afterward.
The README has the per-OS detail.

Build with one or both:

```sh
cargo build --features heif,av
```

## Before opening a pull request

CI runs these three commands on Linux with both features enabled. Running them
locally first is the fastest way to avoid a red build:

```sh
cargo fmt --check
cargo clippy --features heif,av -- -D warnings
cargo test --features heif,av
```

Clippy warnings fail the build, so treat them as errors.

If you do not have the native libraries set up, the same commands without
`--features heif,av` still cover most of the codebase. CI also runs
`cargo check --all-targets` on Windows and macOS with default features, so code
behind `cfg(windows)` or `cfg(target_os = "macos")` needs to compile there.

## GPU tests

Tests that need a wgpu adapter skip when none is available. That keeps
`cargo test` usable on headless machines, but it means a green run does not prove
the GPU paths ran. Set `BLOOM_REQUIRE_GPU=1` to turn a missing adapter into a
failure instead of a silent skip:

```sh
BLOOM_REQUIRE_GPU=1 cargo test --features heif,av wgpu::
```

The value is compared against `1` exactly, so `BLOOM_REQUIRE_GPU=true` does
nothing. CI installs lavapipe (`mesa-vulkan-drivers`) to give the runner a
software Vulkan device and runs with `WGPU_BACKEND=vulkan`.

A few diagnostic probes are marked `#[ignore]` because they allocate several
gigabytes or take minutes. Run them deliberately:

```sh
cargo test --release large_image -- --ignored --nocapture
```

## Style

Comments are file-level `//!` headers only. No inline `//` comments and no `///`
doc comments, except where a `///` is load-bearing, as with the clap argument
help in `src/cli.rs`. Write a header where a reader would otherwise have to
reconstruct the mechanism from the code; a header that restates the filename is
noise. Use American spelling and plain ASCII.

## Dependencies

`cargo audit` runs in CI on every push and on a daily schedule. It fails on
advisories and reports unmaintained or yanked crates as warnings without
failing. If you add a dependency, check it is clean first:

```sh
cargo audit
```

## Reporting bugs

Open an issue with the platform, the release variant you are running
(`-minimal`, `-heif` or the full build), the output of `bloom --version`, and the
file format involved if the problem is tied to one. The issue forms ask for all
of this.
