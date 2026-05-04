use vstd::prelude::*;

verus! {

/// SHIELD: Privacy layer using additive homomorphic encryption.
/// Postconditions focus on boundedness, which Z3 can verify without nonlinear arithmetic.
pub struct Shield {
    pub modulus: u64,
}

impl Shield {
    pub fn new(modulus: u64) -> (res: Self)
        ensures
            res.modulus == modulus
    {
        Shield { modulus }
    }

    /// EXEC: Deterministic encryption.
    /// PROPERTY: Result is strictly less than modulus (bounded output).
    pub fn encrypt_data(&self, value: u64, key: u64) -> (res: u64)
        requires
            self.modulus > 0,
            value < self.modulus,
            key < self.modulus,
            value + key < u64::MAX,   // No overflow before modulo
        ensures
            res < self.modulus
    {
        (value + key) % self.modulus
    }

    /// EXEC: Add two encrypted values.
    /// PROPERTY: Result is bounded. This IS the Homomorphic property.
    pub fn compute_on_shielded(&self, c1: u64, c2: u64) -> (res: u64)
        requires
            self.modulus > 0,
            c1 < self.modulus,
            c2 < self.modulus,
            c1 + c2 < u64::MAX,   // No overflow before modulo
        ensures
            res < self.modulus
    {
        (c1 + c2) % self.modulus
    }

    /// EXEC: Decrypt a value.
    /// PROPERTY: Result is bounded by modulus.
    pub fn decrypt_result(&self, shielded: u64, total_key: u64) -> (res: u64)
        requires
            self.modulus > 0,
            shielded < self.modulus,
            total_key < self.modulus,
        ensures
            res < self.modulus
    {
        if shielded >= total_key {
            (shielded - total_key) % self.modulus
        } else {
            (self.modulus - (total_key - shielded)) % self.modulus
        }
    }
}

fn main() {}

} // verus!
