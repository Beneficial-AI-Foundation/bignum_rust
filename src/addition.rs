pub struct BigNum {
    bits: Vec<bool>, // Store bits in little-endian format (least significant first)
}

impl BigNum {
    pub fn from_u64(mut value: u64) -> Self {
        let mut bits = Vec::new();
        
        if value == 0 {
            bits.push(false);
        } else {
            while value > 0 {
                bits.push(value & 1 == 1);
                value >>= 1;
            }
        }
        
        BigNum { bits }
    }

    pub fn to_u64(&self) -> u64 {
        let mut result = 0u64;
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit && i < 64 {
                result |= 1u64 << i;
            }
        }
        result
    }

    fn normalize(&mut self) {
        // Remove trailing false bits, but keep at least one bit
        while self.bits.len() > 1 && !self.bits.last().unwrap() {
            self.bits.pop();
        }
    }

    /// Add two BigNums and return the result
    pub fn add(&self, other: &BigNum) -> BigNum {
        let max_len = self.bits.len().max(other.bits.len());
        let mut result = Vec::with_capacity(max_len + 1);
        let mut carry = false;
        
        for i in 0..max_len {
            let a = self.bits.get(i).copied().unwrap_or(false);
            let b = other.bits.get(i).copied().unwrap_or(false);
            
            // Full adder logic
            let sum = a ^ b ^ carry;
            let new_carry = (a & b) | (carry & (a ^ b));
            
            result.push(sum);
            carry = new_carry;
        }
        
        // Add final carry if needed
        if carry {
            result.push(true);
        }
        
        let mut bn = BigNum { bits: result };
        bn.normalize();
        bn
    }
}

