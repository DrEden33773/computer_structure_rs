//! Notifications
//!
//! i8::MAX = 2^7-1 = 127
//!
//! i8::MIN = -2^7 = -128

#![allow(dead_code)]

use std::num::Wrapping as wp;

pub fn i8_sub(a: i8, b: i8) -> i8 {
    let a_bits = wp(a as u8);
    let b_bits = wp(b as u8);
    let u8_max_bits = wp(u8::MAX);
    let b_opposite_bits = u8_max_bits - b_bits + wp(1);
    let b_opposite_b_opt_bits = !b_bits + wp(1);
    assert_eq!(b_opposite_bits, b_opposite_b_opt_bits);
    let res_bits = a_bits + b_opposite_b_opt_bits;
    res_bits.0 as i8
}

#[cfg(test)]
mod i8_sub {
    use super::*;

    #[test]
    fn test_i8_sub() {
        let a = i8_sub(67, 32);
        let a_ = (wp(67i8) - wp(32i8)).0;
        let b = i8_sub(32, 89);
        let b_ = (wp(32i8) - wp(89i8)).0;
        assert_eq!(a, a_);
        assert_eq!(b, b_);
    }
}
