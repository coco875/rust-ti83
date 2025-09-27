# Rust TI-84+/83 Premium CE
Experimental repository to run Rust code on the TI‑84+/83 Premium CE (eZ80) by emitting LLVM BC and transcompiling it to C with [llvm-cbe](https://github.com/JuliaHubOSS/llvm-cbe) and compiling with the [CE-Programming toolchain](https://ce-programming.github.io/toolchain/index.html).

# Building
Only macOS and Linux are supported for building.
## Requirements
- Rust (with `cargo` and `rustc`)
- Install the Rust toolchain `nightly-2025-08-01`. See `.github/workflows/ci.yml` for an example setup.

# How it works
It's based on previous work [here](https://github.com/coco875/rustice/tree/old?tab=readme-ov-file#how-it-works). We use the LTO feature to emit LLVM bitcode from Rust, then a custom linker transforms that bitcode into C using [llvm-cbe](https://github.com/JuliaHubOSS/llvm-cbe). I maintain a fork to emit executable artifacts and fix some compilation errors: [the fork](https://github.com/coco875/llvm-cbe). The emitted C code is compiled by the CE-Programming toolchain as usual. Note: the linker ignores most arguments passed to it, but it supports passing C files to enable safe bindings that rely on a custom calling convention or the `uint24_t` type.

# Global note
- I haven't tried other no_std-compatible libraries such as `heapless` or `libm`. If you try them and they work, please open an issue or PR to document it.
- `libcompiler_builtins` and the symbol file are currently ignored because they are not emitted as LLVM bitcode and do not cause issues for now. They may need to be reimplemented later.
- `llvm-cbe` may occasionally emit invalid C. If you encounter this, please open an issue or submit a PR.

# Current limitations
- No `std` support; only `core` and `alloc` are available. Others may work but are untested.
- Not well tested; only the example works for now.

## Alternative approaches considered
- Forking Rust 1.69: Modify Rust 1.69 to use the CE-Programming toolchain’s LLVM 15 fork for direct eZ80 code generation. This is being explored in [ez80-rust](https://github.com/ez80-rust), but requires large-scale changes.
- Custom compiler backend: Add eZ80 support to Cranelift or implement it in a newer LLVM. There is ongoing Z80 work in [cranelift-z80](https://github.com/zlfn/cranelift-z80), and CE-Programming attempted this in their [LLVM fork](https://github.com/CE-Programming/llvm-project), but without success so far.
- Use LLVM bitcode: Emit LLVM bitcode with 1.69 and compile it with the CE-Programming toolchain’s LLVM fork. This failed due to unusual compilation errors; see [this attempt](https://github.com/coco875/rustice/tree/old).