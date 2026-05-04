mod poly_commit;
mod homomorphic_encrypt;
mod vdf;
mod schnorr;
mod merkle;
mod use_case;

use poly_commit::Polynomial;
use homomorphic_encrypt::HomomorphicScheme;
use vdf::VDF;
use schnorr::Schnorr;
use merkle::MerkleTree;
use use_case::run_grand_lottery_demo;

fn main() {
    println!("--- 🧱 The Complete 5-Brick Cryptographic Stack 🧱 ---");
    println!("Moving from verifying data (Identity/Integrity) to computing on it (Efficiency/Privacy/Time).\n");

    // --- 1. Schnorr (The "Identity") ---
    println!("--- 🆔 1. Schnorr (The Identity) ---");
    println!("Concept: Prove WHO authorized the action without revealing your secret.");
    let schnorr = Schnorr::new();
    let priv_key = 42; 
    
    // We must calculate the public key: P = g^x mod p
    // Didactic access helper or just redo math
    let pub_key = {
        let mut res = 1u128;
        let mut b = 5u128; // g
        let mut e = priv_key;
        let p = 1_000_000_007u128;
        while e > 0 {
            if e % 2 == 1 { res = (res * b) % p; }
            b = (b * b) % p;
            e /= 2;
        }
        res as u64
    };
    
    let message = "Authorize Transaction #001";
    
    let signature = schnorr.sign(message, priv_key);
    println!("User: I'm signing '{}' with my private key ({}).", message, priv_key);
    println!("System: Public key is {}. Verifying...", pub_key);
    
    if schnorr.verify(message, signature, pub_key) {
        println!("✅ SUCCESS: Identity verified! This action was authorized by the key owner.\n");
    } else {
        println!("❌ FAILURE: Signature verification failed.\n");
    }

    // --- 2. Merkle Trees (The "Integrity") ---
    println!("--- 🌲 2. Merkle Trees (The Integrity) ---");
    println!("Concept: Organize massive amounts of data into a single, tiny 'Root' hash.");
    let data = vec!["Tx1", "Tx2", "Tx3", "Tx4", "Tx5", "Tx6", "Tx7", "Tx8"];
    let tree = MerkleTree::new(data.clone());
    println!("System: Root hash of {} transactions is {}.", data.len(), tree.root());
    
    let tx_idx = 2; // "Tx3"
    let proof = tree.prove(tx_idx);
    println!("Verifier: Is '{}' part of the block with root {}?", data[tx_idx], tree.root());
    
    // Simulating hash of the leaf
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    use std::hash::{Hash, Hasher};
    data[tx_idx].hash(&mut hasher);
    let leaf_hash = hasher.finish();

    if MerkleTree::verify(tree.root(), leaf_hash, proof) {
        println!("✅ SUCCESS: Integrity proven! We know '{}' is in the set without seeing the rest.\n", data[tx_idx]);
    } else {
        println!("❌ FAILURE: Merkle proof invalid.\n");
    }

    // --- 3. Polynomial Commitments (The "Swiss Army Knife") ---
    println!("--- 🛠️ 3. Polynomial Commitments (The Swiss Army Knife) ---");
    println!("Concept: Commit to a mathematical curve. Proves relationships between data points.");
    let poly = Polynomial::new(vec![10, 5, 2]); // P(x) = 10 + 5x + 2x^2
    let commitment = poly.commit();
    let x = 3;
    let y = poly.evaluate(x);
    println!("Prover: I commit to my curve! Commitment: {}", commitment);
    println!("Verifier: Pick x={}. Prover: P({})={}.", x, x, y);
    if Polynomial::verify(commitment, x, y, &poly) {
        println!("✅ SUCCESS: Evaluation proven! Much more efficient than sending the whole curve.\n");
    }

    // --- 4. Homomorphic Encryption (The "Blindfold Processor") ---
    println!("--- 🗳️ 4. Homomorphic Encryption (The Blindfold Processor) ---");
    println!("Concept: Perform math ON encrypted data without ever decrypting it.");
    let scheme = HomomorphicScheme::new(1000); 
    let v1 = 42; let k1 = 123; let ev1 = scheme.encrypt(v1, k1);
    let v2 = 58; let k2 = 456; let ev2 = scheme.encrypt(v2, k2);
    let res_c = scheme.add_on_ciphertexts(&ev1, &ev2);
    let total_v = scheme.decrypt(res_c, k1 + k2);
    println!("Processor: Added C1 and C2 to get {}.", res_c);
    if total_v == v1 + v2 {
        println!("✅ SUCCESS: Decrypted result is {}. The processor never saw '42' or '58'!\n", total_v);
    }

    // --- 5. Verifiable Delay Functions (The "Proof of Time") ---
    println!("--- ⏳ 5. Verifiable Delay Functions (The Proof of Time) ---");
    println!("Concept: An hourglass that cannot be sped up. Ensures fairness and order.");
    let vdf = VDF::new(1_000_000_007);
    let seed = 12345;
    let steps = 1_000_000;
    let result = vdf.compute(seed, steps);
    println!("System: Sequential computation of {} steps complete.", steps);
    if vdf.verify(seed, steps, result) {
        println!("✅ SUCCESS: Time has passed. Randomness can now be safely generated!\n");
    }

    println!("--- 🚀 Architecture Complete ---");
    println!("Schnorr: Who authorized the action (Privacy/Identity).");
    println!("Merkle: Organizes Massive State (Integrity/Scalability).");
    println!("Polynomials: Proves Data Relationships (Efficiency).");
    println!("Homomorphic: Does Work on hidden data (Computation).");
    println!("VDFs: Ensures Fairness and chronological order (Time).");
    
    // --- 🌍 UNIFIED USE CASE: THE GRAND LOTTERY ---
    run_grand_lottery_demo();
}
