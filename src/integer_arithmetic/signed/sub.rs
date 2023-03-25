//! Notifications
//!
//! i8::MAX = 2^7-1 = 127
//!
//! i8::MIN = -2^7 = -128

#![allow(dead_code)]

use std::num::Wrapping as wp;

pub fn i8_sub(lhs: i8, rhs: i8) -> i8 {
    let lhs_bits = wp(lhs as u8);
    let rhs_bits = wp(rhs as u8);
    let u8_max_bits = wp(u8::MAX);
    let rhs_opposite_bits = u8_max_bits - rhs_bits + wp(1);
    let rhs_opposite_bit_opt_bits = !rhs_bits + wp(1);
    assert_eq!(rhs_opposite_bits, rhs_opposite_bit_opt_bits);
    let res_bits = lhs_bits + rhs_opposite_bit_opt_bits;
    res_bits.0 as i8
}

#[cfg(test)]
mod i8_sub {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_i8_sub() {
        let a = i8_sub(67, 32);
        let a_ = (wp(67i8) - wp(32i8)).0;
        let b = i8_sub(32, 89);
        let b_ = (wp(32i8) - wp(89i8)).0;
        let random_lhs = rand::thread_rng().gen_range(i8::MIN..=i8::MAX);
        let random_rhs = rand::thread_rng().gen_range(i8::MIN..=i8::MAX);
        let c = i8_sub(random_lhs, random_rhs);
        let c_ = (wp(random_lhs) - wp(random_rhs)).0;
        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
    }
}
