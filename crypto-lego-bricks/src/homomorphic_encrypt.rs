/// A simple didactic Additive Homomorphic Encryption scheme.
/// Concepts: Enc(m) = m + k mod M.
/// To add: Enc(m1) + Enc(m2) = (m1 + m2) + (k1 + k2) mod M.
/// The verifier/processor can add the ciphertexts without knowing m1 or m2!
pub struct EncryptedValue {
    pub ciphertext: u64,
    pub key_used: u64, // In a real system, the key stays with the user.
}

pub struct HomomorphicScheme {
    pub modulus: u64,
}

impl HomomorphicScheme {
    pub fn new(modulus: u64) -> Self {
        Self { modulus }
    }

    pub fn encrypt(&self, message: u64, key: u64) -> EncryptedValue {
        EncryptedValue {
            ciphertext: (message + key) % self.modulus,
            key_used: key,
        }
    }

    /// The Magic: The processor adds the ciphertexts!
    /// They don't know the messages, yet they are performing math.
    pub fn add_on_ciphertexts(&self, c1: &EncryptedValue, c2: &EncryptedValue) -> u64 {
        (c1.ciphertext + c2.ciphertext) % self.modulus
    }

    /// The User decrypts the final result using the sum of their keys.
    pub fn decrypt(&self, final_ciphertext: u64, sum_of_keys: u64) -> u64 {
        if final_ciphertext >= sum_of_keys {
            (final_ciphertext - sum_of_keys) % self.modulus
        } else {
            (self.modulus + final_ciphertext - sum_of_keys) % self.modulus
        }
    }
}
