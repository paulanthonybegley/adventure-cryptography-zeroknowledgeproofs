pub struct VDF {
    pub modulus: u64,
}

impl VDF {
    pub fn new(modulus: u64) -> Self {
        Self { modulus }
    }

    pub fn compute(&self, seed: u64, steps: u32) -> u64 {
        let mut res = seed;
        for _ in 0..steps {
            res = (res as u128 * res as u128 % self.modulus as u128) as u64;
        }
        res
    }

    pub fn verify(&self, seed: u64, steps: u32, result: u64) -> bool {
        self.compute(seed, steps) == result
    }
}
