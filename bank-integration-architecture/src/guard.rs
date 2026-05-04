/// GUARD: The Fairness Enforcer.
/// Ensures high-frequency trade/auction fairness via sequential work.
pub struct Guard {
    pub p: u64,
}

impl Guard {
    pub fn new() -> Self {
        Self { p: 1_000_000_007 }
    }

    pub fn run_delay(&self, seed: u64, steps: u32) -> u64 {
        let mut res = seed;
        for _ in 0..steps {
            res = (res as u128 * res as u128 % self.p as u128) as u64;
        }
        res
    }

    pub fn verify_fairness(&self, seed: u64, steps: u32, res: u64) -> bool {
        self.run_delay(seed, steps) == res
    }
}
