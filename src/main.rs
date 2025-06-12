mod bn;

use bn::BigNum;

fn main() {
    println!("BigNum Operations Demo");
    println!("=====================");
    
    // Create some BigNums for testing
    let a = BigNum::from_u64(42);
    let b = BigNum::from_u64(58);
    
    println!("a = {} (decimal: {})", a, a.to_u64());
    println!("b = {} (decimal: {})", b, b.to_u64());
    println!();
    
    // Addition Demo
    println!("Addition:");
    println!("---------");
    let add_result = a.add(&b);
    println!("a + b = {} (decimal: {})", add_result, add_result.to_u64());
    
    // Test carry propagation
    let carry1 = BigNum::from_u64(15); // 1111
    let carry2 = BigNum::from_u64(1);  // 0001
    let carry_result = carry1.add(&carry2);
    println!("{} + {} = {} (decimal: {})",
             carry1, carry2, carry_result, carry_result.to_u64());
    println!();
    
    // Subtraction Demo
    println!("Subtraction:");
    println!("------------");
    let sub_result = b.subtract(&a);
    println!("b - a = {} (decimal: {})", sub_result, sub_result.to_u64());
    
    let large_sub1 = BigNum::from_u64(100);
    let large_sub2 = BigNum::from_u64(37);
    let large_sub_result = large_sub1.subtract(&large_sub2);
    println!("{} - {} = {} (decimal: {})",
             large_sub1, large_sub2, large_sub_result, large_sub_result.to_u64());
    println!();
    
    // Multiplication Demo
    println!("Multiplication:");
    println!("---------------");
    let m1 = BigNum::from_u64(7);
    let m2 = BigNum::from_u64(6);
    let mul_result1 = m1.multiply(&m2);
    println!("{} * {} = {} (decimal: {})", m1, m2, mul_result1, mul_result1.to_u64());

    let m3 = BigNum::from_u64(12);
    let m4 = BigNum::from_u64(15);
    let mul_result2 = m3.multiply(&m4);
    println!("{} * {} = {} (decimal: {})", m3, m4, mul_result2, mul_result2.to_u64());

    // Test multiplication resulting in a larger number
    let m5 = BigNum::from_u64(100);
    let m6 = BigNum::from_u64(100);
    let mul_result3 = m5.multiply(&m6);
    println!("{} * {} = {} (decimal: {})", m5, m6, mul_result3, mul_result3.to_u64());
    println!();
    
    // Division Demo
    println!("Division:");
    println!("---------");
    let div1 = BigNum::from_u64(100);
    let div2 = BigNum::from_u64(7);
    let (quotient1, remainder1) = div1.divide(&div2);
    println!("{} ÷ {} = {} remainder {} (decimal: {} remainder {})",
             div1, div2, quotient1, remainder1, quotient1.to_u64(), remainder1.to_u64());
    
    let div3 = BigNum::from_u64(256);
    let div4 = BigNum::from_u64(16);
    let (quotient2, remainder2) = div3.divide(&div4);
    println!("{} ÷ {} = {} remainder {} (decimal: {} remainder {})",
             div3, div4, quotient2, remainder2, quotient2.to_u64(), remainder2.to_u64());
    
    // Test convenience methods
    let div_only = div1.div(&div2);
    let mod_only = div1.modulo(&div2);
    println!("Using convenience methods: {} ÷ {} = {}, {} mod {} = {}",
             div1, div2, div_only.to_u64(), div1, div2, mod_only.to_u64());
    println!();
    
    // Bit shift operations
    println!("Bit Shift Operations:");
    println!("--------------------");
    let shift_num = BigNum::from_u64(5); // 101 in binary
    println!("Original: {} (decimal: {})", shift_num, shift_num.to_u64());
    
    let left_shifted = shift_num.shl(2);
    println!("Left shift by 2:  {} (decimal: {})", left_shifted, left_shifted.to_u64());
    
    let right_shifted = left_shifted.shr(1);
    println!("Right shift by 1: {} (decimal: {})", right_shifted, right_shifted.to_u64());
    println!();
    
    // Comparison operations
    println!("Comparison Operations:");
    println!("---------------------");
    let comp1 = BigNum::from_u64(42);
    let comp2 = BigNum::from_u64(37);
    let comp3 = BigNum::from_u64(42);
    
    use std::cmp::Ordering;
    match comp1.cmp(&comp2) {
        Ordering::Greater => println!("{} > {}", comp1, comp2),
        Ordering::Less => println!("{} < {}", comp1, comp2),
        Ordering::Equal => println!("{} = {}", comp1, comp2),
    }
    
    match comp1.cmp(&comp3) {
        Ordering::Greater => println!("{} > {}", comp1, comp3),
        Ordering::Less => println!("{} < {}", comp1, comp3),
        Ordering::Equal => println!("{} = {}", comp1, comp3),
    }
    println!();
    
    // Edge cases
    println!("Edge Cases:");
    println!("-----------");
    let zero = BigNum::from_u64(0);
    let one = BigNum::from_u64(1);
    
    println!("Zero test: {} (is_zero: {})", zero, zero.is_zero());
    println!("One test: {} (is_zero: {})", one, one.is_zero());
    
    let zero_add = zero.add(&BigNum::from_u64(42));
    println!("0 + 42 = {} (decimal: {})", zero_add, zero_add.to_u64());
    
    let zero_mul = zero.multiply(&BigNum::from_u64(42));
    println!("0 * 42 = {} (decimal: {})", zero_mul, zero_mul.to_u64());
    
    let div_by_one = BigNum::from_u64(123).div(&one);
    println!("123 ÷ 1 = {} (decimal: {})", div_by_one, div_by_one.to_u64());
    
    println!();
    println!("Demo completed successfully!");
}