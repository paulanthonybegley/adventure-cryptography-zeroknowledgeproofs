use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// ENGINE: The Batcher.
/// Combines massive request lists into a single proof.
pub struct Engine {
    pub levels: Vec<Vec<u64>>,
}

impl Engine {
    pub fn new(requests: Vec<&str>) -> Self {
        let mut leaves = Vec::new();
        for r in requests {
            let mut hasher = DefaultHasher::new();
            r.hash(&mut hasher);
            leaves.push(hasher.finish());
        }
        let mut levels = vec![leaves.clone()];
        let mut current = leaves;
        while current.len() > 1 {
            let mut next = Vec::new();
            for chunk in current.chunks(2) {
                let mut hasher = DefaultHasher::new();
                chunk[0].hash(&mut hasher);
                if chunk.len() > 1 { chunk[1].hash(&mut hasher); }
                else { chunk[0].hash(&mut hasher); }
                next.push(hasher.finish());
            }
            levels.push(next.clone());
            current = next;
        }
        Self { levels }
    }

    pub fn get_commitment(&self) -> u64 {
        self.levels.last().unwrap()[0]
    }

    /// Polynomial Efficiency: Proving that 10,000 transactions sum to X
    pub fn prove_balance_relationship(&self, values: Vec<u64>) -> u64 {
        // Didactic: Return a hash of the relationship
        let mut hasher = DefaultHasher::new();
        values.hash(&mut hasher);
        hasher.finish()
    }
}
