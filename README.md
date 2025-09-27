# Rust TI-84+/83 Premium CE
Experimental repository to run Rust code on the TI‑84+/83 Premium CE (eZ80) by emitting LLVM BC and transcompiling it to C with [llvm-cbe](https://github.com/JuliaHubOSS/llvm-cbe) and compiling with the [CE-Programming toolchain](https://ce-programming.github.io/toolchain/index.html).

# Building
Only macOS and Linux are supported for building.
## Requirements
- Rust (with `cargo` and `rustc`)
- Install the Rust toolchain `nightly-2025-08-01`. See `.github/workflows/ci.yml` for an example setup.

# How it works
It's based on previous work [here](https://github.com/coco875/rustice/tree/old?tab=readme-ov-file#how-it-works). We use the LTO feature to emit LLVM bitcode from Rust, then a custom linker transforms that bitcode into C using [llvm-cbe](https://github.com/JuliaHubOSS/llvm-cbe). I maintain a fork to emit executable artifacts and fix some compilation errors: [the fork](https://github.com/coco875/llvm-cbe). The emitted C code is compiled by the CE-Programming toolchain as usual. Note: the linker ignores most arguments passed to it, but it supports passing C files to enable safe bindings that rely on a custom calling convention or the `uint24_t` type.