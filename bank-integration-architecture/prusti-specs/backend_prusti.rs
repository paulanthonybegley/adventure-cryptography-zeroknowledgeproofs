use prusti_contracts::*;

/// BACKEND: The Legacy Bank Database Simulator.
/// Final destination for the verified cryptographic bundle.
pub struct Backend {
    pub vault_balance: u64,
}

impl Backend {
    #[requires(true)]
    #[ensures(self.vault_balance == initial_balance)]
    pub fn new(initial_balance: u64) -> Self {
        Self { vault_balance: initial_balance }
    }

    #[requires(true)]
    #[ensures(self.vault_balance == old(self.vault_balance))]
    pub fn process_verified_bundle(&mut self, root: u64, proof_valid: bool) {
        if proof_valid {
            println!("Legacy Bank System: VALIDATING bundle root: {}", root);
            println!("Legacy Bank System: CRITERIA MET. Updating internal ledger...");
        } else {
            println!("Legacy Bank System: REJECTED! Inconsistent cryptographic proof.");
        }
    }
}
