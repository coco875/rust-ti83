# Rust TI-84+/83 Premium CE
Experimental repo to run Rust code on the TI‑84+/83-Premium-CE (ez80) by emitting llvm and compiling with the [CE-Programming toolchain](https://ce-programming.github.io/toolchain/index.html).

# Building
Only macOS and Linux are supported for building.
## Requirements
- Rust (with `cargo` and `rustc`)
- LLVM (with `llvm-dis`, if possible version 15)
- Have the toolchain nightly-2023-03-03 of Rust installed.
See `.github/workflows/ci.yml` for an example of setup.
Don't forget to compile the `ti83-fake-linker.rs` file with `rustc ti83-fake-linker.rs -o ti83-fake-linker`.

# How it works

## Historical context
This project draws inspiration from [Rust-CE](https://github.com/maddymakesgames/Rust-CE), which was the first attempt to target Rust for TI calculators. However, Rust-CE has become outdated and no longer works with recent Rust versions. Additionally, it relied on `cargo make` for compilation, which is less well integrated with the standard Cargo ecosystem.

## First approach (deprecated)
The initial implementation was inspired by both [Rust-CE](https://github.com/maddymakesgames/Rust-CE) and [n64-project-template](https://github.com/rust-n64/n64-project-template).

**Process:**
1. Target `wasm32-unknown-unknown` architecture
2. Emit LLVM IR using `--emit=llvm-ir` 
3. Configure a custom runner in `.cargo/config.toml` to handle linking and `.8xp` generation
4. Use a custom executable to process LLVM IR files and generate `.8xp` files with the CE-Programming toolchain

**Limitations:**
- Modern Rust emits LLVM IR that's too recent for the toolchain (which uses LLVM 15)
- Had to use Rust 1.69.0 (the last version using LLVM 15)
- This version had poor support for adding/removing libraries
- The approach needed a more appropriate target specification

## Current approach
The current implementation leverages Rust nightly features for better control and flexibility, specifically using the nightly version of Rust 1.69.

**Key improvements:**
- **Custom target:** Created a custom target specification (initially based on wasm32, later simplified to resemble `armv4t-unknown-none`)
- **Standard library building:** Supports `-Zbuild-std=core` for building the core library from source
- **Link-time optimization:** Uses LTO with a custom profile and `-Clinker-plugin-lto` to emit LLVM bitcode (.bc) files instead of object files
- **Custom linker:** Uses a fake linker specified in the `linker` field for final linking

**Process flow:**
1. Compile Rust code with custom target
2. Generate LLVM bitcode files through LTO
3. Process bitcode files to create `.8xp` executable

## Current limitations

### LLVM IR processing
- **Call convention patching:** Still requires disassembling with `llvm-dis` to patch calling conventions for TI-OS functions
- **Alternative approach:** Direct binary patching is theoretically possible but hasn't been attempted

### Compiler built-ins
- **Missing support:** Rust compiler built-ins are currently ignored
- **Format incompatibility:** Built-ins aren't provided in LLVM bitcode format

### Complex LLVM features
- **Unsupported conventions:** The toolchain doesn't support certain LLVM features like `fastcc` calling convention
- **Library restrictions:** This prevents building `alloc`, so collections (Vec, HashMap, etc.) are not supported

### Type system limitations
- **24-bit integers:** Rust doesn't natively support i24 integers, which are common on the ez80 architecture
- **Workaround:** Sometimes requires using structs with Rust calling conventions to emit i24 integers
- **Drawbacks:** This approach is not ideal and can be cumbersome