use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Didactic Polynomial Commitment for Efficient Vector Proofs.
pub struct PolynomialCommitment {
    pub coefficients: Vec<u64>,
}

impl PolynomialCommitment {
    pub fn new(coefficients: Vec<u64>) -> Self {
        Self { coefficients }
    }

    pub fn commit(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.coefficients.hash(&mut hasher);
        hasher.finish()
    }

    pub fn evaluate(&self, x: u64) -> u64 {
        let mut result = 0;
        let mut x_pow = 1;
        for &c in &self.coefficients {
            result += c * x_pow;
            x_pow *= x;
        }
        result
    }

    pub fn verify(commitment: u64, x: u64, y: u64, candidate: &PolynomialCommitment) -> bool {
        candidate.commit() == commitment && candidate.evaluate(x) == y
    }
}
