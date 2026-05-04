use vstd::prelude::*;

verus! {

/// FRONTEND: Identity layer using Schnorr-style modular exponentiation.
/// The postconditions here prove structural properties that the SMT solver can discharge.
pub struct Frontend {
    pub p: u64,
    pub g: u64,
}

impl Frontend {
    pub fn new() -> (res: Self)
        ensures
            res.p == 1_000_000_007u64,
            res.g == 5u64
    {
        Frontend { p: 1_000_000_007, g: 5 }
    }

    /// SPEC: Abstract modular exponentiation for proof purposes.
    pub open spec fn spec_pow_mod(self, base: nat, exp: nat) -> nat {
        pow_mod_spec(base, exp, self.p as nat)
    }

    /// EXEC: Iterative modular exponentiation.
    /// VERIFIER PROPERTY: Result is always < p (key safety bound).
    pub fn pow_mod(&self, base: u64, exp: u64) -> (res: u64)
        requires
            self.p > 1,
            self.p < 0x40000000u64,  // p < 2^30: ensures (p-1)*(p-1) < 2^60 < u128::MAX
        ensures
            res < self.p
    {
        let mut result: u128 = 1;
        let mut b: u128 = (base % self.p) as u128;
        let mut e = exp;
        while e > 0
            invariant
                self.p > 1,
                self.p < 0x40000000u64,
                result < self.p as u128,
                b < self.p as u128,
            decreases e
        {
            if e % 2 == 1 {
                result = result.wrapping_mul(b) % self.p as u128;
            }
            b = b.wrapping_mul(b) % self.p as u128;
            e /= 2;
        }
        result as u64
    }

    /// EXEC: Verify a customer request.
    pub fn verify_customer(&self, signature: (u64, u64), pub_key: u64) -> (res: bool)
        requires
            self.p > 1,
            self.p < 0x40000000u64,
    {
        let (r, s) = signature;
        let e: u64 = 42;
        let left = self.pow_mod(self.g, s);
        let right_pow = self.pow_mod(pub_key, e);
        let rp = right_pow as u128;
        let rr = (r % self.p) as u128;
        let right = (rr.wrapping_mul(rp) % self.p as u128) as u64;
        left == right
    }

    /// INVARIANT: pow_mod output is always bounded by the modulus.  
    pub proof fn proof_pub_key_bounded(self, base: nat, exp: nat)
        requires
            self.p > 0,
        ensures
            pow_mod_spec(base, exp, self.p as nat) < self.p as nat
    {
        proof_pow_mod_bounded(base, exp, self.p as nat);
    }
}

// Abstract spec function for modular exponentiation (used in proofs)
pub open spec fn pow_mod_spec(base: nat, exp: nat, p: nat) -> nat
    decreases exp
{
    if p == 0 {
        0nat
    } else if exp == 0 {
        1nat % p
    } else {
        (pow_mod_spec(base, (exp - 1) as nat, p) * base) % p
    }
}

/// TOP-LEVEL INVARIANT: pow_mod_spec result is always bounded by p.
pub proof fn proof_pow_mod_bounded(base: nat, exp: nat, p: nat)
    requires p > 0,
    ensures pow_mod_spec(base, exp, p) < p
    decreases exp
{
    if exp == 0 {
        // 1nat % p < p for p > 0
    } else {
        proof_pow_mod_bounded(base, (exp - 1) as nat, p);
        // (x * base) % p < p follows from modular arithmetic
    }
}

fn main() {}

} // verus!
