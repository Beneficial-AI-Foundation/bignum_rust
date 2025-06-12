use crate::BigNum;

impl BigNum {
    /// Divide self by other, returns (quotient, remainder)
    pub fn divide(&self, other: &BigNum) -> (BigNum, BigNum) {
        if other.is_zero() {
            panic!("Division by zero");
        }
        
        if self.is_zero() {
            return (BigNum::from_u64(0), BigNum::from_u64(0));
        }
        
        use std::cmp::Ordering;
        match self.cmp(other) {
            Ordering::Less => {
                // self < other, so quotient is 0 and remainder is self
                return (BigNum::from_u64(0), self.clone());
            }
            Ordering::Equal => {
                // self == other, so quotient is 1 and remainder is 0
                return (BigNum::from_u64(1), BigNum::from_u64(0));
            }
            Ordering::Greater => {
                // Proceed with long division
            }
        }
        
        // Long division algorithm
        let mut quotient_bits = vec![false; self.bits.len()];
        let mut remainder = BigNum::from_u64(0);

        // Process bits from MSB to LSB
        for i in (0..self.bits.len()).rev() {
            // Shift remainder left by 1 and add current bit
            remainder = remainder.shl(1);
            if self.bits[i] {
                remainder = remainder.add(&BigNum::from_u64(1));
            }

            // If remainder >= divisor, subtract and set quotient bit
            if remainder.cmp(other) != Ordering::Less {
                remainder = remainder.subtract(other);
                quotient_bits[i] = true;
            }
        }
        
        let mut quotient = BigNum { bits: quotient_bits };
        quotient.normalize();
        remainder.normalize();
        (quotient, remainder)
    }
    
    /// Divide self by other, returns quotient only
    pub fn div(&self, other: &BigNum) -> BigNum {
        self.divide(other).0
    }
    
    /// Get remainder of self divided by other
    pub fn modulo(&self, other: &BigNum) -> BigNum {
        self.divide(other).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_divide_simple() {
        let a = BigNum::from_u64(15);
        let b = BigNum::from_u64(3);
        let (quotient, remainder) = a.divide(&b);
        assert_eq!(quotient.to_u64(), 5);
        assert_eq!(remainder.to_u64(), 0);
    }
    
    #[test]
    fn test_divide_with_remainder() {
        let a = BigNum::from_u64(17);
        let b = BigNum::from_u64(5);
        let (quotient, remainder) = a.divide(&b);
        assert_eq!(quotient.to_u64(), 3);
        assert_eq!(remainder.to_u64(), 2);
    }
    
    #[test]
    fn test_divide_smaller_by_larger() {
        let a = BigNum::from_u64(3);
        let b = BigNum::from_u64(10);
        let (quotient, remainder) = a.divide(&b);
        assert_eq!(quotient.to_u64(), 0);
        assert_eq!(remainder.to_u64(), 3);
    }
    
    #[test]
    fn test_divide_equal_numbers() {
        let a = BigNum::from_u64(42);
        let b = BigNum::from_u64(42);
        let (quotient, remainder) = a.divide(&b);
        assert_eq!(quotient.to_u64(), 1);
        assert_eq!(remainder.to_u64(), 0);
    }
    
    #[test]
    fn test_divide_by_one() {
        let a = BigNum::from_u64(123);
        let b = BigNum::from_u64(1);
        let (quotient, remainder) = a.divide(&b);
        assert_eq!(quotient.to_u64(), 123);
        assert_eq!(remainder.to_u64(), 0);
    }
    
    #[test]
    fn test_div_and_modulo() {
        let a = BigNum::from_u64(25);
        let b = BigNum::from_u64(7);
        
        let quotient = a.div(&b);
        let remainder = a.modulo(&b);
        
        assert_eq!(quotient.to_u64(), 3);
        assert_eq!(remainder.to_u64(), 4);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        let a = BigNum::from_u64(10);
        let b = BigNum::from_u64(0);
        a.divide(&b);
    }
}