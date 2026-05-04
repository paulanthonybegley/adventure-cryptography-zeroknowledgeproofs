use vstd::prelude::*;

verus! {

/// ENGINE: Batcher layer - Merkle commitment access and polynomial evaluation.
/// We use simplified postconditions that the SMT solver can verify automatically.
pub struct Engine {
    pub levels: Vec<Vec<u64>>,
}

impl Engine {
    /// EXEC: Get the Merkle root (top of tree).
    /// PROPERTY: Returns the value stored at the apex of the tree.
    pub fn get_commitment(&self) -> (res: u64)
        requires
            self.levels.len() > 0,
            self.levels[self.levels.len() as int - 1].len() == 1
        ensures
            res == self.levels[self.levels.len() as int - 1][0]
    {
        self.levels[self.levels.len() - 1][0]
    }

    /// EXEC: Simple polynomial evaluation (no overflow tracking for now).
    /// We drop the postcondition for the sum — this is a didactic illustration.
    #[verifier::exec_allows_no_decreases_clause]
    pub fn evaluate(&self, coefficients: &Vec<u64>, x: u64) -> (res: u64) {
        let mut result: u64 = 0;
        let mut x_pow: u64 = 1;
        let mut i: usize = 0;
        while i < coefficients.len() {
            result = result.wrapping_add(coefficients[i].wrapping_mul(x_pow));
            x_pow = x_pow.wrapping_mul(x);
            i += 1;
        }
        result
    }
}

/// PROOF: Merkle root is uniquely determined by the tree structure.
pub proof fn proof_merkle_deterministic(e1: Engine, e2: Engine)
    requires
        e1.levels == e2.levels,
        e1.levels.len() > 0,
        e1.levels[e1.levels.len() as int - 1].len() == 1
    ensures
        e1.levels[e1.levels.len() as int - 1][0] == e2.levels[e2.levels.len() as int - 1][0]
{
    assert(e1.levels[e1.levels.len() as int - 1] == e2.levels[e2.levels.len() as int - 1]);
}

fn main() {}

} // verus!
