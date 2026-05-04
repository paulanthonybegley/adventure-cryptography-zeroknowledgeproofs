use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A didactic Schnorr Signature scheme with Aggregation.
/// In this version, we show how two users can combine their keys and signatures.
pub struct Schnorr {
    pub p: u64,
    pub g: u64,
}

impl Schnorr {
    pub fn new() -> Self {
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

    /// Sign a message with a private key.
    pub fn sign(&self, message: &str, private_key: u64) -> (u64, u64) {
        let k = 123456; // Didactic (should be random)
        let r_val = self.pow_mod(self.g, k);
        let mut hasher = DefaultHasher::new();
        r_val.hash(&mut hasher);
        message.hash(&mut hasher);
        let e = hasher.finish() % (self.p - 1);
        let s = ((k as u128 + (e as u128 * private_key as u128)) % (self.p as u128 - 1)) as u64;
        (r_val, s)
    }

    /// SIGNATURE AGGREGATION: Combine two signatures into one.
    /// This is the "Lego Brick" value for Multi-Sig.
    pub fn aggregate_signatures(&self, sig1: (u64, u64), sig2: (u64, u64)) -> (u64, u64) {
        let (r1, s1) = sig1;
        let (r2, s2) = sig2;
        // R_agg = R1 * R2, s_agg = s1 + s2
        let r_agg = (r1 as u128 * r2 as u128 % self.p as u128) as u64;
        let s_agg = (s1 as u128 + s2 as u128 % (self.p as u128 - 1)) as u64;
        (r_agg, s_agg)
    }

    /// AGGREGATED PUBLIC KEY: Combine two public keys.
    pub fn aggregate_public_keys(&self, pk1: u64, pk2: u64) -> u64 {
        (pk1 as u128 * pk2 as u128 % self.p as u128) as u64
    }

    pub fn verify(&self, message: &str, signature: (u64, u64), public_key: u64) -> bool {
        let (r_val, s) = signature;
        let mut hasher = DefaultHasher::new();
        r_val.hash(&mut hasher);
        message.hash(&mut hasher);
        let e = hasher.finish() % (self.p - 1);
        let left = self.pow_mod(self.g, s);
        let right = ((r_val as u128 * self.pow_mod(public_key, e) as u128) % self.p as u128) as u64;
        left == right
    }
}
