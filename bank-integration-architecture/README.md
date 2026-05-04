# 🏦 Bank Trust-less Bridge (Educational)

A didactic Rust project illustrating the **"Trust-less Bridge"** architecture. This middle layer allows Legacy Banks to integrate five high-level cryptographic building blocks to achieve better security, privacy, and throughput.

## 🏗️ The 5 Architectural Layers

1.  📱 **Frontend**: Identity via Schnorr (Password-less authorization).
2.  ⚙️ **Engine**: Batching via Merkle Trees (Legacy core scalability).
3.  🛡️ **Shield**: Privacy via Homomorphic Encryption (PII-protected auditing).
4.  ⚔️ **Guard**: Fairness via VDFs (Anti-frontrunning trade auctions).
5.  💾 **Backend**: Legacy database integration (Verification-only updates).

---

## 🚀 How to Run

```bash
cargo run
```

The console output provides a narrative demonstration of the **Bank Trust-less Bridge** lifecycle.

## 📁 Project Structure

- `src/frontend.rs`: Customer identity and Schnorr authorization.
- `src/engine.rs`: Merkle-based request batching and polynomial proof logic.
- `src/shield.rs`: Homomorphic encryption for private bank audits.
- `src/guard.rs`: VDF-based sequential delay for trade fairness.
- `src/backend.rs`: Integration with the legacy bank ledger.
- `src/main.rs`: Orchestrates the 5-layer architectural demo.
- `education-bank-bridge.md`: Detailed architectural guide.

---
> [!IMPORTANT]
> **Educational Disclaimer:** These implementations are simplified for learning purposes and are **NOT** secure for production.
