# 🌉 Bridge Verus Migration Strategy

This document outlines the architectural reasoning behind our dual-mode development of the Bank Trust-less Bridge, and explicitly details the migration steps required to move from an educational sandbox to a native production framework.

---

## 🤔 Why is the Verified Code Isolated?

Currently, the files in `verus-specs/` (e.g., `frontend_verus.rs`) each contain an empty `fn main() {}` at their bottom. They run completely isolated from the standard `cargo run` executing in `src/main.rs`. This is intentional for two reasons:

### 1. Toolchain Limitations (The "Standalone" Target)
Verus operates by intercepting the Rust compiler. When executing `./verus <file.rs>`, the toolchain requires an entry point to begin parsing. Injecting an empty `fn main() {}` tricks the Verus compiler into treating the mathematical specification file as a standalone binary package. This lets it instantly read and verify the math top-to-bottom without requiring a complex `Cargo.toml` workspace configuration linking everything to a `lib.rs`.

### 2. Pedagogical Clarity (Side-by-Side Learning)
For learning cryptography, cognitive load matters. Right now, this repository contains "Normal Rust" inside `src/` and "Mathematically Proven Rust" inside `verus-specs/`. 
Having them strictly separated allows developers to cleanly read the underlying business logic in execution, and then swap to the `.verus` file to study the strict geometric bounds, `invariant`, and `decreases` statements required to mathematically prove that exact same code. 

If we merged them prematurely in the education phase, the sheer volume of SMT constraints would obscure the flow of the simulated Bridge Demo.

---

## 🚀 The Migration Plan: Upgrading to Production

If we were to take this repository from an educational demonstration into a true Production Banking infrastructure, we *would* inextricably merge the proofs into the core binary. Here are the exact architectural migration steps (and why no complex logic changes are fundamentally required):

### Step 1: Replace `src/` modules natively
Instead of maintaining parallel `verus-specs/` files, we would physically overwrite the files inside `src/` (like `src/frontend.rs`) with the highly annotated Verus code.

### Step 2: Add `vstd` as a Cargo Dependency
The Verus Standard Library (`vstd`) provides the mathematical types like `nat`, `int`, and `Seq`. We would modify `Cargo.toml` to actively fetch this via Git as a standard dependency:
```toml
[dependencies]
vstd = { git = "https://github.com/verus-lang/verus" }
```

### Step 3: Strip the dummy `main` functions
We would delete `fn main() {}` from the bottom of each module. Once merged into `src/`, they organically revert to acting as standard `pub struct` library modules hooked together by `src/main.rs`.

### Step 4: Use a Verified Cargo Wrapper
In production, instead of awkwardly running `./verus file.rs` manually through a `Makefile`, you use the global integration wrapper **`cargo-verus`**. 

When executing `cargo verus build`, the framework natively natively traverses your standard workspace tree. It algebraically verifies every mathematical proof globally, safely erases the `spec` and `proof` blocks from the AST, and asks standard `rustc` to compile a highly optimized, standard Mach-O/ELF binary under the hood.

**The ultimate guarantee is achieved:** The exact code you formally verify is the exact binary executing on your hardware.
