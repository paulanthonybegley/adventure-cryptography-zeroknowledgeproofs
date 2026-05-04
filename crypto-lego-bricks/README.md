# 🧱 The 5 Cryptographic Lego Bricks (Educational)

A simple Rust project illustrating the "Complete Cryptographic Stack." These five building blocks move us from simply verifying data to computing on it while it stays hidden and ensuring fairness in time.

## 🆔 1. Schnorr (The "Identity")
**The Concept:** Prove **Who** authorized an action without revealing your private key.
- **Used for:** Digital signatures in Bitcoin (Taproot), Polkadot, and modern ZK-proofs.

## 🌲 2. Merkle Trees (The "Integrity")
**The Concept:** Organize a **Massive State** into a single root hash. Prove a piece of data is in the set with a tiny "proof."
- **Used for:** Blockchain state storage, light clients, and Git.

## 🛠️ 3. Polynomial Commitments (The "Swiss Army Knife")
**The Concept:** Commit to a mathematical **function** (a curve). Prove the value of the curve at any point without revealing the rest.
- **Used for:** KZG Commitments, Ethereum Danksharding, and PLONK-based ZKPs.

## 🗳️ 4. Homomorphic Encryption (The "Blindfold Processor")
**The Concept:** Perform **math on encrypted data** without ever decrypting it.
- **Used for:** Private AI, medical data processing, and private voting.

## ⏳ 5. Verifiable Delay Functions (The "Proof of Time")
**The Concept:** A digital **hourglass** that cannot be parallelized. Ensures fairness and chronological order.
- **Used for:** Unbiased randomness beacons and blockchain leader selection.

---

## 🌌 The Unified Use Case: The Grand Intergalactic Lottery

The project includes a cohesive demonstration in `src/use_case.rs` where all 5 bricks work together:

1. **Schnorr**: Alice signs her ticket entry to prove her identity.
2. **Merkle Trees**: The entire pool of 1,000,000 tickets is committed to a single Root hash (Locked State).
3. **Homomorphic Encryption**: Entry fees are summed while encrypted, protecting the prize pool privacy.
4. **Polynomial Commitments**: The system proves Alice's ticket is a "Winning Point" on a secret curve without revealing the curve.
5. **VDF**: A "Proof of Time" is generated after the pool is locked to ensure the random winner selection is unbiased.

---

## 🚀 How to Run

```bash
cargo run
```

The console output provides a narrative demonstration of each concept.

## 📁 Project Structure

- `src/schnorr.rs`: Didactic Schnorr signature ($s = k + e \cdot x$).
- `src/merkle.rs`: Simple binary Merkle Tree and path proofs.
- `src/poly_commit.rs`: Polynomial evaluation and hash-based commitment.
- `src/homomorphic_encrypt.rs`: Toy additive homomorphic encryption.
- `src/vdf.rs`: Sequential squaring VDF.
- `src/main.rs`: Orchestrates the full 5-brick educational demo.

---
> [!IMPORTANT]
> **Educational Disclaimer:** These implementations are simplified for learning purposes and are **NOT** secure for production. Real-world versions use advanced mathematics, elliptic curves, and formal proofs.
