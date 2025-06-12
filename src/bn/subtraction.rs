use crate::BigNum;

impl BigNum {
    /// Subtract other from self (assumes self >= other)
    pub fn subtract(&self, other: &BigNum) -> BigNum {
        let max_len = self.bits.len().max(other.bits.len());
        let mut result = Vec::with_capacity(max_len);
        let mut borrow = false;

        for i in 0..max_len {
            let a = self.bits.get(i).copied().unwrap_or(false);
            let b = other.bits.get(i).copied().unwrap_or(false);

            // Subtract with borrow: diff = a - b - borrow
            let diff = a as i8 - b as i8 - borrow as i8;
            if diff >= 0 {
                result.push(diff != 0);
                borrow = false;
            } else {
                result.push(true); // 1 - 1 - 1 = -1, which is 1 with borrow
                borrow = true;
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
    fn test_subtract() {
        let a = BigNum::from_u64(10);
        let b = BigNum::from_u64(3);
        let result = a.subtract(&b);
        assert_eq!(result.to_u64(), 7);
    }
}