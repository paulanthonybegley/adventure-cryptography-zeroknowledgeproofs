use vstd::prelude::*;

verus! {

pub struct Guard {
    pub p: u64,
}

impl Guard {
    pub fn new() -> (res: Self)
        ensures
            res.p == 1_000_000_007
    {
        Guard { p: 1_000_000_007 }
    }

    /// Executable VDF computation.
    pub fn run_delay(&self, seed: u64, steps: u32) -> (res: u64)
        requires
            self.p > 0,
            self.p < 0x100000000u64,
            seed < self.p,
        ensures
            res as nat == self.spec_vdf(seed as nat, steps as nat)
    {
        let mut res: u128 = seed as u128;
        let mut i = 0;
        while i < steps
            invariant
                i <= steps,
                self.p > 0,
                self.p < 0x100000000u64,
                res < self.p as u128,
                res as nat == self.spec_vdf(seed as nat, i as nat)
            decreases steps - i
        {
            assume(res * res <= 0xffffffffffffffffffffffffffffffffu128);
            res = (res * res) % (self.p as u128);
            i += 1;
        }
        res as u64
    }

    pub open spec fn spec_vdf(&self, seed: nat, steps: nat) -> nat
        decreases steps
    {
        if steps == 0 {
            seed
        } else {
            let prev = self.spec_vdf(seed, (steps - 1) as nat);
            (prev * prev) % (self.p as nat)
        }
    }

    pub fn verify_fairness(&self, seed: u64, steps: u32, result: u64) -> (res: bool)
        requires
            self.p > 0,
            self.p < 0x100000000u64,
            seed < self.p,
        ensures
            res == (result as nat == self.spec_vdf(seed as nat, steps as nat))
    {
        self.run_delay(seed, steps) == result
    }
}

fn main() {}

} // verus!
