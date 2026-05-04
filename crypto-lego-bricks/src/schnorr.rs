use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A didactic Schnorr Signature scheme for educational purposes.
/// Key Concept: $s = k + e \cdot x$
pub struct Schnorr {
    pub p: u64, // Prime modulus
    pub g: u64, // Generator
}

impl Schnorr {
    pub fn new() -> Self {
        // Didactic values (not secure!)
        Self { p: 1_000_000_007, g: 5 }
    }

    fn pow_mod(&self, base: u64, exp: u64) -> u64 {
        let mut res = 1u128;
        let mut b = (base % self.p) as u128;
        let mut e = exp;
        while e > 0 {
            if e % 2 == 1 { res = (res * b) % self.p as u128; }
            b = (b * b) % self.p as u128;
            e /= 2;
        }
        res as u64
    }

    /// Prove Who authorized the action.
    /// Returns (R, s) as the signature.
    pub fn sign(&self, message: &str, private_key: u64) -> (u64, u64) {
        let k = 123456; // In a real system, this MUST be random!
        let r_val = self.pow_mod(self.g, k);
        
        let mut hasher = DefaultHasher::new();
        r_val.hash(&mut hasher);
        message.hash(&mut hasher);
        let e = hasher.finish() % (self.p - 1);
        
        // s = (k + e * x) mod (p-1)
        // Use u128 to prevent overflow during (e * private_key)
        let s = ((k as u128 + (e as u128 * private_key as u128)) % (self.p as u128 - 1)) as u64;
        (r_val, s)
    }

    /// Verifier checks the signature against the public key.
    pub fn verify(&self, message: &str, signature: (u64, u64), public_key: u64) -> bool {
        let (r_val, s) = signature;
        
        let mut hasher = DefaultHasher::new();
        r_val.hash(&mut hasher);
        message.hash(&mut hasher);
        let e = hasher.finish() % (self.p - 1);
        
        // Check: g^s == R * P^e
        let left = self.pow_mod(self.g, s);
        let right = ((r_val as u128 * self.pow_mod(public_key, e) as u128) % self.p as u128) as u64;
        
        left == right
    }
}
