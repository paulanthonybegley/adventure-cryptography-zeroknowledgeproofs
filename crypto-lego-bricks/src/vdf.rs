/// A simple didactic Verifiable Delay Function (VDF).
/// Concepts: Sequential squaring g^(2^T) mod N.
/// This cannot be parallelized because each step depends on the previous result.
pub struct VDF {
    pub modulus: u64,
}

impl VDF {
    pub fn new(modulus: u64) -> Self {
        Self { modulus }
    }

    /// Compute the result of the VDF after a certain number of steps.
    /// This is the "Work" that takes time.
    pub fn compute(&self, seed: u64, steps: u32) -> u64 {
        let mut result = seed;
        for _ in 0..steps {
            // result = result^2 mod modulus
            result = (result * result) % self.modulus;
        }
        result
    }

    /// Simple verification: a verifier must also run the steps to verify.
    /// (In real VDFs like Wesolowski, there's a short proof, 
    /// but for this didactic version, we just focus on the non-parallel nature.)
    pub fn verify(&self, seed: u64, steps: u32, claimed_result: u64) -> bool {
        self.compute(seed, steps) == claimed_result
    }
}
