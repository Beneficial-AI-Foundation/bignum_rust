/// Represents a big number as a vector of bits (LSB first)
#[derive(Debug, Clone, PartialEq)]
pub struct BigNum {
    pub(crate) bits: Vec<bool>,
}

impl BigNum {
    /// Create a new BigNum from a u64 value
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
    
    /// Create a new BigNum from a bit vector (LSB first)
    pub fn from_bits(bits: Vec<bool>) -> Self {
        BigNum { bits }
    }
    
    /// Get the length in bits
    pub fn len(&self) -> usize {
        self.bits.len()
    }
    
    /// Remove leading zeros
    pub(crate) fn normalize(&mut self) {
        while self.bits.len() > 1 && !self.bits.last().unwrap() {
            self.bits.pop();
        }
    }

    /// Convert to u64 (truncates if too large)
    pub fn to_u64(&self) -> u64 {
        let mut result = 0u64;
        for (i, &bit) in self.bits.iter().enumerate() {
            if i >= 64 { break; }
            if bit {
                result |= 1u64 << i;
            }
        }
        result
    }
    
    /// Check if the BigNum is zero
    pub fn is_zero(&self) -> bool {
        self.bits.iter().all(|&bit| !bit)
    }
    
    /// Left shift by n positions (multiply by 2^n)
    pub fn shl(&self, n: usize) -> BigNum {
        if self.is_zero() || n == 0 {
            return self.clone();
        }
        
        let mut result = vec![false; n];
        result.extend(&self.bits);
        
        let mut bn = BigNum { bits: result };
        bn.normalize();
        bn
    }
    
    /// Right shift by n positions (divide by 2^n)
    pub fn shr(&self, n: usize) -> BigNum {
        if self.is_zero() || n == 0 {
            return self.clone();
        }
        
        if n >= self.bits.len() {
            return BigNum::from_u64(0);
        }
        
        let result = self.bits[n..].to_vec();
        if result.is_empty() {
            return BigNum::from_u64(0);
        }
        
        let mut bn = BigNum { bits: result };
        bn.normalize();
        bn
    }
    
    /// Compare two BigNums: returns Ordering
    pub fn cmp(&self, other: &BigNum) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        
        // First compare by length (after normalization)
        let mut self_normalized = self.clone();
        let mut other_normalized = other.clone();
        self_normalized.normalize();
        other_normalized.normalize();
        
        match self_normalized.bits.len().cmp(&other_normalized.bits.len()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // Same length, compare bit by bit from MSB to LSB
                for i in (0..self_normalized.bits.len()).rev() {
                    let self_bit = self_normalized.bits[i];
                    let other_bit = other_normalized.bits[i];
                    match self_bit.cmp(&other_bit) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl std::fmt::Display for BigNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Simple binary representation (MSB first for display)
        let binary: String = self.bits.iter().rev().map(|&b| if b { '1' } else { '0' }).collect();
        write!(f, "0b{}", binary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from_u64() {
        let bn = BigNum::from_u64(5);
        assert_eq!(bn.bits, vec![true, false, true]); // 101 in binary (LSB first)
    }
    
    #[test]
    fn test_is_zero() {
        let zero = BigNum::from_u64(0);
        let non_zero = BigNum::from_u64(1);
        assert!(zero.is_zero());
        assert!(!non_zero.is_zero());
    }
    
    #[test]
    fn test_shl() {
        let a = BigNum::from_u64(5); // 101
        let result = a.shl(2);       // Should be 10100 = 20
        assert_eq!(result.to_u64(), 20);
    }
    
    #[test]
    fn test_shr() {
        let a = BigNum::from_u64(20); // 10100
        let result = a.shr(2);        // Should be 101 = 5
        assert_eq!(result.to_u64(), 5);
    }
    
    #[test]
    fn test_cmp() {
        let a = BigNum::from_u64(10);
        let b = BigNum::from_u64(5);
        let c = BigNum::from_u64(10);
        
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(b.cmp(&a), std::cmp::Ordering::Less);
        assert_eq!(a.cmp(&c), std::cmp::Ordering::Equal);
    }
}