use vstd::prelude::*;

verus! {

pub struct Backend {
    pub vault_balance: u64,
}

impl Backend {
    pub fn new(initial_balance: u64) -> (res: Self)
        ensures
            res.vault_balance == initial_balance
    {
        Backend { vault_balance: initial_balance }
    }

    /// Executable state update with formal verification.
    pub fn process_verified_bundle(&mut self, root: u64, proof_valid: bool)
        requires
            proof_valid == true,
        ensures
            self.vault_balance == old(self).vault_balance, // Balance doesn't change in this didactic log step
            self.spec_is_safe()
    {
        if proof_valid {
            // Log verification success
        }
    }

    pub open spec fn spec_is_safe(&self) -> bool {
        self.vault_balance >= 0
    }
}

fn main() {}

} // verus!
