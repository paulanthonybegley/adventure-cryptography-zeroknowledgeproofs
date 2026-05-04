# Running Prusti Specifications

This directory contains Rust code annotated with [Prusti](https://github.com/viperproject/prusti-dev) formal verification specifications.

## Prerequisites

1.  **Rust Toolchain**: Ensure you have a recent version of Rust installed.
2.  **Prusti**: You must install the Prusti toolchain. Follow the instructions at [Prusti Installation Guide](https://viperproject.github.io/prusti-dev/user-guide/install.html).
    - Typically, this involve running:
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/viperproject/prusti-dev/master/scripts/install.sh | sh
      ```
3.  **Dependencies**: The specifications depend on the `prusti-contracts` crate.

## Running the Verification

To verify the specifications, you can use `prusti-rustc` directly on each file, or integrate them into a Cargo project.

### Method 1: Direct Verification
Navigate to this directory and run `prusti-rustc` on the module you want to verify:
```bash
prusti-rustc backend_prusti.rs --edition 2024
```

### Method 2: Running the Simulation
To run the `main_prusti.rs` simulation, you need to provide the `prusti-contracts` dependency. The easiest way is to add it to your `Cargo.toml`:

```toml
[dependencies]
prusti-contracts = "0.1"
```

Then you can run the simulation:
```bash
cargo run --bin main_prusti
```
(Note: You may need to move these files into your `src/` directory or adjust your `[[bin]]` configurations in `Cargo.toml`).

## What is Being Verified?

- **Backend**: Ensures that the ledger state (vault balance) matches initialization and is preserved during bundle processing.
- **Engine**: Verifies that the Merkle Root returned is the actual root of the generated tree.
- **Frontend**: Ensures modular arithmetic stays within the bounds of the prime modulus `p`.
- **Guard**: Verifies the functional identity of the VDF logic.
- **Shield**: Checks the correctness of the decryption logic as a modular inverse of encryption.
