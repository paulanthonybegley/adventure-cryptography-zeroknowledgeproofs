use crate::schnorr::Schnorr;
use crate::merkle::MerkleTree;
use crate::poly_commit::Polynomial;
use crate::homomorphic_encrypt::HomomorphicScheme;
use crate::vdf::VDF;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn run_grand_lottery_demo() {
    println!("\n========================================================");
    println!("🌌 WELCOME TO THE GRAND INTERGALACTIC LOTTERY 🌌");
    println!("========================================================\n");

    // 1. Identity (Schnorr) - Alice enters the lottery
    println!("--- 🆔 STEP 1: IDENTITY (SCHNORR) ---");
    let schnorr = Schnorr::new();
    let alice_priv = 1337;
    let alice_pub = 168695277; // g^alice_priv mod p (manually verified for g=5, p=1000000007)
    let ticket_msg = "Alice's Winning Ticket #42";
    let alice_sig = schnorr.sign(ticket_msg, alice_priv);
    
    println!("Alice: I'm signing my ticket entry with my private key.");
    if schnorr.verify(ticket_msg, alice_sig, alice_pub) {
        println!("✅ System: Alice's identity confirmed via Schnorr Signature. Entry accepted!\n");
    }

    // 2. Integrity (Merkle) - Committing the entire pool
    println!("--- 🌲 STEP 2: INTEGRITY (MERKLE) ---");
    let all_tickets = vec![
        ticket_msg, 
        "Bob's Ticket #13", 
        "Charlie's Ticket #7", 
        "Dave's Ticket #99"
    ];
    let pool_tree = MerkleTree::new(all_tickets.clone());
    let pool_root = pool_tree.root();
    println!("System: All {} tickets are hashed into a single Merkle Root: {}.", all_tickets.len(), pool_root);
    println!("System: The lottery pool is now LOCKED. No more entries allowed.\n");

    // 3. Privacy (Homomorphic) - Calculating the Prize Pool
    println!("--- 🗳️ STEP 3: PRIVACY (HOMOMORPHIC) ---");
    let scheme = HomomorphicScheme::new(10000);
    let alice_fee = 500; let alice_k = 111;
    let bob_fee = 300; let bob_k = 222;
    
    let enc1 = scheme.encrypt(alice_fee, alice_k);
    let enc2 = scheme.encrypt(bob_fee, bob_k);
    
    println!("System: Entry fees are encrypted. Alice sent {}, Bob sent {}.", enc1.ciphertext, enc2.ciphertext);
    let prize_pool_enc = scheme.add_on_ciphertexts(&enc1, &enc2);
    let total_prize = scheme.decrypt(prize_pool_enc, alice_k + bob_k);
    println!("System: The Homomorphic Processor calculated the prize pool while BLINDFOLDED!");
    println!("✅ System: Total Prize Pool (Decrypted by Authority): {} tokens.\n", total_prize);

    // 4. Efficiency (Polynomials) - Checking the Winning Point
    println!("--- 🛠️ STEP 4: EFFICIENCY (POLYNOMIALS) ---");
    // The "Winning Curve" is defined by the system's secret parameters P(x)
    let winning_curve = Polynomial::new(vec![42, 7, 3]); // Some coefficients
    let commitment = winning_curve.commit();
    let winning_x = 42; // Alice's ticket number
    let winning_y = winning_curve.evaluate(winning_x);
    
    println!("System: The Winning Curve commitment is {}.", commitment);
    println!("System: Checking if Alice's ticket number {} lies on the winner's point...", winning_x);
    if Polynomial::verify(commitment, winning_x, winning_y, &winning_curve) {
        println!("✅ System: POINT VERIFIED. Alice's coordinate matches the winning formula!\n");
    }

    // 5. Fairness (VDF) - Proving that time has passed before drawing
    println!("--- ⏳ STEP 5: FAIRNESS (VDF) ---");
    let vdf = VDF::new(1_000_000_007);
    let block_seed = pool_root % 1_000_000_007; // Seeded by the Merkle Root
    let delay_steps = 500_000;
    
    println!("System: Starting the VDF 'Hourglass' to finalize the result.");
    println!("System: This prevents anyone from 'sniping' a ticket after seeing the draw.");
    let final_randomness = vdf.compute(block_seed, delay_steps);
    
    if vdf.verify(block_seed, delay_steps, final_randomness) {
        println!("✅ System: VDF Proof of Time verified. Draw is UNBIASED and sequential.");
        println!("System: The final randomness {} confirms the winner!\n", final_randomness);
    }

    println!("========================================================");
    println!("🏆 THE ARCHITECTURE IS COMPLETE 🏆");
    println!("Who signed? Alice (Schnorr)");
    println!("Did entries change? No (Merkle)");
    println!("Secret Prize Pool sum? Done (Homomorphic)");
    println!("Winning math proven? Yes (Polynomials)");
    println!("Is it rigged? No, time has passed (VDF)");
    println!("========================================================\n");
}
