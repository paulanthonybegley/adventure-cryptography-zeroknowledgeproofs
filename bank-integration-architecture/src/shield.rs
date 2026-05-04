/// SHIELD: The Privacy Guard.
/// Processes sensitive data (like credit scores) while it's encrypted.
pub struct Shield {
    pub modulus: u64,
}

impl Shield {
    pub fn new(modulus: u64) -> Self {
        Self { modulus }
    }

    pub fn encrypt_data(&self, value: u64, key: u64) -> u64 {
        (value + key) % self.modulus
    }

    /// Compute credit score on encrypted data
    pub fn compute_on_shielded(&self, c1: u64, c2: u64) -> u64 {
        (c1 + c2) % self.modulus
    }

    pub fn decrypt_result(&self, shielded: u64, total_key: u64) -> u64 {
        if shielded >= total_key {
            (shielded - total_key) % self.modulus
        } else {
            (self.modulus + shielded - total_key) % self.modulus
        }
    }
}
