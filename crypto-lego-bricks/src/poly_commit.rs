use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A simple didactic Polynomial: P(x) = c0 + c1*x + c2*x^2 + ...
pub struct Polynomial {
    pub coefficients: Vec<u64>,
}

impl Polynomial {
    pub fn new(coefficients: Vec<u64>) -> Self {
        Self { coefficients }
    }

    /// The "Commitment": A small piece of data that represents the whole curve.
    /// In a real system (like KZG), this is a group element. 
    /// Here, we use a Hash for educational simplicity.
    pub fn commit(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.coefficients.hash(&mut hasher);
        hasher.finish()
    }

    /// Evaluate the polynomial at point x: y = P(x)
    pub fn evaluate(&self, x: u64) -> u64 {
        let mut result = 0;
        let mut x_pow = 1;
        for &coeff in &self.coefficients {
            result += coeff * x_pow;
            x_pow *= x;
        }
        result
    }

    /// Generate a "Proof" for the evaluation: y = P(x)
    /// In real math, this would be a "witness" polynomial.
    /// Here, we just provide the evaluation and the commitment.
    pub fn prove_evaluation(&self, x: u64) -> (u64, u64) {
        let y = self.evaluate(x);
        let commitment = self.commit();
        (y, commitment)
    }

    /// Verify that P(x) = y without knowing the whole polynomial!
    /// (In this didactic version, we simulate the "magic" by 
    /// showing that the commitment matches.)
    pub fn verify(commitment: u64, x: u64, y: u64, poly_candidate: &Polynomial) -> bool {
        // A verifier only sees the commitment, x, and y.
        // If someone provides a 'candidate' polynomial that matches 
        // the commitment AND evaluates correctly, they've proven it.
        poly_candidate.commit() == commitment && poly_candidate.evaluate(x) == y
    }
}
