use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A didactic Merkle Tree for educational purposes.
/// Key Concept: Organize Massive State into a single root hash.
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub leaves: Vec<u64>,
    pub levels: Vec<Vec<u64>>,
}

impl MerkleTree {
    pub fn new(data: Vec<&str>) -> Self {
        let mut leaves = Vec::new();
        for item in data {
            let mut hasher = DefaultHasher::new();
            item.hash(&mut hasher);
            leaves.push(hasher.finish());
        }

        let mut levels = vec![leaves.clone()];
        let mut current_level = leaves.clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in current_level.chunks(2) {
                let mut hasher = DefaultHasher::new();
                chunk[0].hash(&mut hasher);
                if chunk.len() > 1 {
                    chunk[1].hash(&mut hasher);
                } else {
                    chunk[0].hash(&mut hasher); // Duplicate for odd number
                }
                next_level.push(hasher.finish());
            }
            levels.push(next_level.clone());
            current_level = next_level;
        }

        Self { leaves, levels }
    }

    pub fn root(&self) -> u64 {
        self.levels.last().unwrap()[0]
    }

    /// Prove Integrity: Provide the sibling hashes and their position.
    pub fn prove(&self, index: usize) -> Vec<(u64, bool)> {
        let mut proof = Vec::new();
        let mut idx = index;
        for level in &self.levels {
            if level.len() <= 1 { break; }
            let is_right = idx % 2 == 1;
            let sibling_idx = if is_right { idx - 1 } else { idx + 1 };
            
            if sibling_idx < level.len() {
                proof.push((level[sibling_idx], is_right));
            } else {
                proof.push((level[idx], is_right)); 
            }
            idx /= 2;
        }
        proof
    }

    pub fn verify(root: u64, leaf_hash: u64, proof: Vec<(u64, bool)>) -> bool {
        let mut current = leaf_hash;
        for (sibling, is_right) in proof {
            let mut hasher = DefaultHasher::new();
            if is_right {
                sibling.hash(&mut hasher);
                current.hash(&mut hasher);
            } else {
                current.hash(&mut hasher);
                sibling.hash(&mut hasher);
            }
            current = hasher.finish();
        }
        current == root
    }
}
