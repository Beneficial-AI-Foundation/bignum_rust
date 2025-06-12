use crate::BigNum;

impl BigNum {
    /// Multiply two BigNums and return the result
    pub fn multiply(&self, other: &BigNum) -> BigNum {
        // Handle zero cases
        if self.is_zero() || other.is_zero() {
            return BigNum::from_u64(0);
        }
        
        // Initialize result with zeros
        let result_len = self.bits.len() + other.bits.len();
        let mut result = vec![false; result_len];
        
        // Grade-school multiplication algorithm
        for i in 0..self.bits.len() {
            if self.bits[i] {
                let mut carry = false;
                
                for j in 0..other.bits.len() {
                    let pos = i + j;
                    if pos >= result.len() { break; }
                    
                    let a = result[pos];
                    let b = other.bits[j];
                    
                    // Add with carry
                    let sum = a ^ b ^ carry;
                    let new_carry = (a & b) | (carry & (a ^ b));
                    
                    result[pos] = sum;
                    carry = new_carry;
                }
                
                // Propagate final carry
                let mut pos = i + other.bits.len();
                while carry && pos < result.len() {
                    let sum = result[pos] ^ carry;
                    carry = result[pos] & carry;
                    result[pos] = sum;
                    pos += 1;
                }
            }
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
    fn test_multiply_simple() {
        let a = BigNum::from_u64(5); // 101
        let b = BigNum::from_u64(3); // 011
        let result = a.multiply(&b);
        assert_eq!(result.to_u64(), 15); // 5 * 3 = 15
    }
    
    #[test]
    fn test_multiply_by_zero() {
        let a = BigNum::from_u64(42);
        let b = BigNum::from_u64(0);
        let result = a.multiply(&b);
        assert_eq!(result.to_u64(), 0);
    }
    
    #[test]
    fn test_multiply_by_one() {
        let a = BigNum::from_u64(42);
        let b = BigNum::from_u64(1);
        let result = a.multiply(&b);
        assert_eq!(result.to_u64(), 42);
    }
    
    #[test]
    fn test_multiply_larger() {
        let a = BigNum::from_u64(12); // 1100
        let b = BigNum::from_u64(15); // 1111
        let result = a.multiply(&b);
        assert_eq!(result.to_u64(), 180); // 12 * 15 = 180
    }
    
    #[test]
    fn test_multiply_powers_of_two() {
        let a = BigNum::from_u64(8);  // 1000
        let b = BigNum::from_u64(16); // 10000
        let result = a.multiply(&b);
        assert_eq!(result.to_u64(), 128); // 8 * 16 = 128
    }
}