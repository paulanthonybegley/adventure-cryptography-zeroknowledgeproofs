mod schnorr;
mod merkle;
mod poly_commit;
mod homomorphic_encrypt;
mod vdf;

use schnorr::Schnorr;
use merkle::MerkleTree;
use poly_commit::PolynomialCommitment;
use homomorphic_encrypt::HomomorphicAudit;
use vdf::VDF;

fn main() {
    println!("\n========================================================");
    println!("💸 WELCOME TO THE HIGH-SPEED BATCH VAULT 💸");
    println!("========================================================\n");

    // 1. Identity (Schnorr Aggregation) - Multi-Sig Authorization
    println!("--- 🆔 STEP 1: MULTI-SIG IDENTITY (SCHNORR) ---");
    let schnorr = Schnorr::new();
    let alice_priv = 123; let alice_pub = 445368006; // g^alice_priv mod p
    let bob_priv = 456; let bob_pub = 328005886; // g^bob_priv mod p
    
    let msg = "Authorize Batch #999";
    let sig_alice = schnorr.sign(msg, alice_priv);
    let sig_bob = schnorr.sign(msg, bob_priv);

    println!("Alice and Bob: We are both signing the batch authorization.");
    let agg_sig = schnorr.aggregate_signatures(sig_alice, sig_bob);
    let agg_pub = schnorr.aggregate_public_keys(alice_pub, bob_pub);

    if schnorr.verify(msg, agg_sig, agg_pub) {
        println!("✅ System: AGGREGATED Signature Verified! Two signers, one signature on-chain.\n");
    }

    // 2. Integrity (Merkle Tree) - Batching 1,000s of Payments
    println!("--- 🌲 STEP 2: BATCH INTEGRITY (MERKLE) ---");
    let payments = vec!["Pay $100 to Eve", "Pay $50 to Frank", "Pay $200 to Grace", "Pay $10 to Heidi"];
    let batch_tree = MerkleTree::new(payments.clone());
    println!("System: {} payments batched into Merkle Root: {}.", payments.len(), batch_tree.root());
    
    let p_idx = 2; // Grace's payment
    let proof = batch_tree.prove(p_idx);
    println!("Vendor (Grace): Was my payment '{}' included in the batch?", payments[p_idx]);
    
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    use std::hash::{Hash, Hasher};
    payments[p_idx].hash(&mut hasher);
    if MerkleTree::verify(batch_tree.root(), hasher.finish(), proof) {
        println!("✅ System: Payment verified! Inclusion proof is valid.\n");
    }

    // 3. Efficiency (Polynomial Commitments) - Constant-Sized Proofs
    println!("--- 🛠️ STEP 3: EFFICIENT PROOFS (POLYNOMIALS) ---");
    // Proof that index X paid Y amount using a curve P(index) = amount
    let amount_poly = PolynomialCommitment::new(vec![100, 50, 200, 10]); 
    let commitment = amount_poly.commit();
    let index = 2; // Grace
    let amount = amount_poly.evaluate(index);
    println!("Auditor: Prove that payment #{} was exactly ${} tokens.", index, amount);
    if PolynomialCommitment::verify(commitment, index, amount, &amount_poly) {
        println!("✅ System: Constant-sized proof verified! Scalable lookup for huge batches.\n");
    }

    // 4. Privacy (Homomorphic Encryption) - The Secret Audit
    println!("--- 🗳️ STEP 4: PRIVATE AUDIT (HOMOMORPHIC) ---");
    let audit = HomomorphicAudit::new(100000);
    let a1 = 100; let k1 = 777; let e1 = audit.encrypt(a1, k1);
    let a2 = 50; let k2 = 888; let e2 = audit.encrypt(a2, k2);
    
    println!("System: Payments are encrypted. Individual amounts are hidden.");
    let total_enc = audit.sum_encrypted(&e1, &e2);
    let total_actual = audit.decrypt(&total_enc, k1 + k2);
    println!("Auditor: Summing encrypted outflows... Result (Decrypted): ${}.", total_actual);
    if total_actual == a1 + a2 {
        println!("✅ System: Audit complete. Total outflows matched without revealing individual payees.\n");
    }

    // 5. Fairness (VDF) - Anti-Frontrunning
    println!("--- ⏳ STEP 5: FAIR ORDERING (VDF) ---");
    let vdf = VDF::new(1_000_000_007);
    let seed = batch_tree.root() % 1_000_000_007;
    let steps = 400_000;
    println!("System: Running the 'Sequencer' VDF to finalize transaction order.");
    let final_res = vdf.compute(seed, steps);
    if vdf.verify(seed, steps, final_res) {
        println!("✅ System: Batch finalized. Order is mathematically fair and sequential.\n");
    }

    println!("========================================================");
    println!("🏆 THE BATCH VAULT ARCHITECTURE IS COMPLETE 🏆");
    println!("Multi-Sig: Aggregated & Private (Schnorr)");
    println!("Batching: Scalable & Tamper-proof (Merkle)");
    println!("Verification: Instant & Constant-sized (Polynomials)");
    println!("Audit: Private & Mathematical (Homomorphic)");
    println!("Fairness: Non-predictable & Sequential (VDF)");
    println!("========================================================\n");
}
