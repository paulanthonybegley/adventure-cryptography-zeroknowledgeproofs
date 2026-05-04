use prusti_contracts::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// FRONTEND: The Identity Collector.
/// Proves customer authorization via Schnorr without sending passwords to the bank.
pub struct Frontend {
    pub p: u64,
    pub g: u64,
}

impl Frontend {
    pub fn new() -> Self {
        Self { p: 1_000_000_007, g: 5 }
    }

    #[requires(self.p > 0)]
    #[ensures(result < self.p)]
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

    /// Sign the banking request (e.g., "Withdraw $500")
    #[requires(self.p > 1)]
    pub fn authorize_request(&self, request: &str, priv_key: u64) -> (u64, u64) {
        let k = 12345; // Didactic
        let r_val = self.pow_mod(self.g, k);
        let mut hasher = DefaultHasher::new();
        r_val.hash(&mut hasher);
        request.hash(&mut hasher);
        let e = hasher.finish() % (self.p - 1);
        let s = ((k as u128 + (e as u128 * priv_key as u128)) % (self.p as u128 - 1)) as u64;
        (r_val, s)
    }

    pub fn verify_customer(&self, request: &str, signature: (u64, u64), pub_key: u64) -> bool {
        let (r_val, s) = signature;
        let mut hasher = DefaultHasher::new();
        r_val.hash(&mut hasher);
        request.hash(&mut hasher);
        let e = hasher.finish() % (self.p - 1);
        let left = self.pow_mod(self.g, s);
        let right = ((r_val as u128 * self.pow_mod(pub_key, e) as u128) % self.p as u128) as u64;
        left == right
    }
}
