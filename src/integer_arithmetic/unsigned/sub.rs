//! Notifications
//!
//! u8::MAX = 2^8-1 = 255
//!
//! u8::MIN = 0

#![allow(dead_code)]

use std::num::Wrapping as wp;

pub fn u8_sub(lhs: u8, rhs: u8) -> u8 {
    let lhs = wp(lhs);
    let rhs = wp(rhs);
    let u8_max = wp(u8::MAX);
    let rhs_opposite = u8_max + wp(1) - rhs; // looks familiar to Two's Complex
    let rhs_opposite_bit_opt = !rhs + wp(1); // looks familiar to Two's Complex
    assert_eq!(rhs_opposite, rhs_opposite_bit_opt);
    let res = lhs + rhs_opposite;
    res.0
}

#[cfg(test)]
mod u8_sub {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_u8_sub() {
        let a = u8_sub(15, 2);
        let b = u8_sub(233, 11);
        let c = u8_sub(2, 15);
        let a_ = (wp(15u8) - wp(2u8)).0;
        let b_ = (wp(233u8) - wp(11u8)).0;
        let c_ = (wp(2u8) - wp(15u8)).0;
        let random_lhs = rand::thread_rng().gen_range(u8::MIN..=u8::MAX);
        let random_rhs = rand::thread_rng().gen_range(u8::MIN..=u8::MAX);
        let d = u8_sub(random_lhs, random_rhs);
        let d_ = (wp(random_lhs) - wp(random_rhs)).0;
        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
        assert_eq!(d, d_);
    }
}
