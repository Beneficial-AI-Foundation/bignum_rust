use crate::BigNum;

impl BigNum {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_simple() {
        let a = BigNum::from_u64(5); // 101
        let b = BigNum::from_u64(3); // 011
        let result = a.add(&b);
        assert_eq!(result.to_u64(), 8); // 1000
    }
    
    #[test]
    fn test_add_with_carry() {
        let a = BigNum::from_u64(15); // 1111
        let b = BigNum::from_u64(1);  // 0001
        let result = a.add(&b);
        assert_eq!(result.to_u64(), 16); // 10000
    }
    
    #[test]
    fn test_add_different_lengths() {
        let a = BigNum::from_u64(255); // 11111111
        let b = BigNum::from_u64(1);   // 00000001
        let result = a.add(&b);
        assert_eq!(result.to_u64(), 256); // 100000000
    }
}