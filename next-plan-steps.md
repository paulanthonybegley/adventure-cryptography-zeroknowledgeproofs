# 🗺️ Master Plan: Advanced Cryptography Implementations

This document serves as the architectural roadmap and educational curriculum for the next phase of the `lego-crypt-foundation-zkp` project. We will transition from foundational primitives into the cutting-edge of modern cryptography, building three distinct modules.

---

## 1. Zero-Knowledge Virtual Machines (ZKVMs)

### 🎯 Objective
Migrate from manual polynomial commitments (our current Engine) to a generalized ZKVM where standard Rust code generates its own Zero-Knowledge Proof.

### 📚 Educational Focus
Understand the difference between **circuit-specific ZKPs** (where every algorithm requires custom math) and **ZKVMs** (where an emulator executes standard instruction sets, like RISC-V, and the emulator itself is what gets mathematically proven).

### 🛠️ Implementation Steps
1. **Initialize a RISC Zero Project:** 
   Use the `cargo risczero new` toolchain to create a multi-crate workspace (host, methods, and core).
2. **Write the Guest Code (The ZK Circuit):**
   Instead of writing math, we write a standard Rust program (e.g., verifying a Sudoku board or executing a bank policy check) inside the `guest` module.
3. **Write the Host Code (The Prover/Verifier):**
   - The Prover runs the guest code through the RISC Zero executor, generating a cryptographic `Receipt`.
   - The Verifier (the Bank) accepts the `Receipt` and the `ImageID` (the hash of the expected Rust binary) and calls `receipt.verify()`.
4. **Integration:** Let the Trust-less Bridge Engine emit a RISC Zero Receipt instead of a manual Merkle root.

---

## 2. Advanced Verifiable Delay Functions (Wesolowski)

### 🎯 Objective
Upgrade our Guard's sequential squaring algorithm $\left(g^{2^T} \pmod M\right)$ from a computationally intensive puzzle into a true VDF featuring *Non-Interactive Proofs of Exponentiation* (NI-PoE).

### 📚 Educational Focus
Learn how **Groups of Unknown Order** (like RSA moduli where $N = p \cdot q$ but $p$ and $q$ are thrown away) prevent participants from taking shortcuts. Learn why the Verifier shouldn't have to repeat the Prover's $T$ steps, but should only need $1$ step to verify.

### 🛠️ Implementation Steps
1. **The Setup:** Generate an RSA Modulus $N$ without revealing its prime factors.
2. **The Prover (Evaluation):** 
   Calculate the delay $y = g^{2^T} \pmod N$. This takes exactly $T$ sequential steps.
3. **The Proof Generation (Wesolowski):**
   Using the Fiat-Shamir heuristic, hash $(g, y)$ to derive a prime challenge $l$. The prover calculates a quotient proof $\pi = g^q \pmod N$. 
4. **The Verifier:**
   The Bank receives $(y, \pi)$. It calculates the same challenge $l$, and then remarkably verifies the entire hourglass calculation instantly by checking: $\pi^l \cdot g^r \equiv y \pmod N$.

---

## 3. Fully Homomorphic Encryption (Lattice Math)

### 🎯 Objective
Replace the Shield's additive-only homomorphic encryption with a foundational Lattice-based encryption scheme (like a toy version of BFV/BGV) allowing both additions *and* multiplications on ciphertext.

### 📚 Educational Focus
Transition from prime-field modular arithmetic to **Lattice Cryptography** and **Learning With Errors (LWE)**. Understand how "noise" secures the ciphertexts, how operations increase noise, and why "bootstrapping" or "relinearization" is the hardest problem in cryptography.

### 🛠️ Implementation Steps
1. **Lattice Primitives:** Build functions for polynomial ring arithmetic (adding and multiplying arrays of constants bounded by a modulus $q$).
2. **LWE Keys & Noise:** Implement key generation that injects a small, random "error polynomial" to secure the cipher.
3. **Encryption & Decryption:** Construct ciphertexts as pairs of polynomials $(c_0, c_1)$ and verify that decryption successfully rounds away the injected noise to reveal the message.
4. **Homomorphic Multiplication:** Implement the complex tensor product of two ciphertexts. Multiply $(c_0, c_1) \times (d_0, d_1)$ to yield a 3-part ciphertext $(e_0, e_1, e_2)$, then prove that decrypting this expanded text yields the product of the original plaintexts!
