# 💸 Crypto Payment Batcher (Educational)

A didactic Rust project illustrating **Payment Batching** and **Multi-Signature** vaults using the 5 cryptographic building blocks of modern decentralized systems.

If the **Galactic Lottery** was about fair games, the **Payment Batcher** is about high-speed, private, and verifiable financial architecture.

## 🧱 The 5 Bricks of the Batcher

1.  🆔 **Schnorr signatures** for Signature Aggregation (Multi-Sig simplicity).
2.  🌲 **Merkle Trees** for Batch Integrity (thousands of payments in one root).
3.  🛠️ **Polynomial Commitments** for Efficient Proofs (constant-sized lookup).
4.  🗳️ **Homomorphic Encryption** for Private Auditing (secret sum of outflows).
5.  ⏳ **Verifiable Delay Functions** for Fair Ordering (anti-frontrunning).

---

## 🚀 How to Run

```bash
cargo run
```

The console output provides a narrative demonstration of the **High-Speed Batch Vault**.

## 📁 Project Structure

- `src/schnorr.rs`: Didactic Schnorr with signature and public key aggregation.
- `src/merkle.rs`: Merkle Tree and path proofs for payment inclusion.
- `src/poly_commit.rs`: Polynomial evaluation for constant-sized vector proofs.
- `src/homomorphic_encrypt.rs`: Additive homomorphic encryption for private accounting.
- `src/vdf.rs`: Sequential squaring VDF for batch sequencing.
- `src/main.rs`: Orchestrates the "High-Speed Batch Vault" demo.
- `education-payment-batching.md`: Detailed educational guide.

---
> [!IMPORTANT]
> **Educational Disclaimer:** These implementations are simplified for learning purposes and are **NOT** secure for production.
