#![allow(dead_code)]

use std::num::Wrapping as wp;

pub fn u8_sub(a: u8, b: u8) -> u8 {
    let a = wp(a);
    let b = wp(b);
    let u8_max = wp(u8::MAX);
    let b_opposite = u8_max + wp(1) - b;
    let res = a + b_opposite;
    res.0
}

#[cfg(test)]
mod u8_sub {
    use super::*;

    #[test]
    fn test_u8_sub() {
        let a = u8_sub(15, 2);
        let b = u8_sub(233, 11);
        let c = u8_sub(2, 15);
        let a_ = (wp(15u8) - wp(2u8)).0;
        let b_ = (wp(233u8) - wp(11u8)).0;
        let c_ = (wp(2u8) - wp(15u8)).0;
        assert_eq!(a, a_);
        assert_eq!(b, b_);
        assert_eq!(c, c_);
    }
}
