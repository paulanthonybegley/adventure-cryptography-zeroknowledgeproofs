pub struct EncryptedAmount {
    pub ciphertext: u64,
}

pub struct HomomorphicAudit {
    pub modulus: u64,
}

impl HomomorphicAudit {
    pub fn new(modulus: u64) -> Self {
        Self { modulus }
    }

    pub fn encrypt(&self, amount: u64, key: u64) -> EncryptedAmount {
        EncryptedAmount { ciphertext: (amount + key) % self.modulus }
    }

    pub fn sum_encrypted(&self, c1: &EncryptedAmount, c2: &EncryptedAmount) -> EncryptedAmount {
        EncryptedAmount { ciphertext: (c1.ciphertext + c2.ciphertext) % self.modulus }
    }

    pub fn decrypt(&self, encrypted: &EncryptedAmount, total_key: u64) -> u64 {
        if encrypted.ciphertext >= total_key {
            (encrypted.ciphertext - total_key) % self.modulus
        } else {
            (self.modulus + encrypted.ciphertext - total_key) % self.modulus
        }
    }
}
