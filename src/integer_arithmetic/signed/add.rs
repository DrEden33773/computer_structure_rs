//! Notifications
//!
//! i8::MAX = 2^7-1 = 127
//!
//! i8::MIN = -2^7 = -128

#![allow(dead_code)]

use std::num::Wrapping as wp;

#[derive(PartialEq, Debug)]
pub enum ErrType<T> {
    PositiveOverflow(T),
    NegativeOverflow(T),
}

pub fn i8_add_ok(lhs: i8, rhs: i8) -> Result<i8, ErrType<i8>> {
    let sum = (wp(lhs) + wp(rhs)).0;
    if lhs > 0 && rhs > 0 && sum < 0 {
        return Err(ErrType::PositiveOverflow(sum));
    }
    if lhs < 0 && rhs < 0 && sum > 0 {
        return Err(ErrType::NegativeOverflow(sum));
    }
    Ok(sum)
}

#[cfg(test)]
mod i8_add {
    use super::*;

    #[test]
    fn test_i8_add_ok() {
        let ans1 = i8_add_ok(3, 8).ok().unwrap();
        let ans2 = i8_add_ok(78, 122).err().unwrap();
        let ans3 = i8_add_ok(-78, -122).err().unwrap();
        assert_eq!(ans1, 3i8 + 8i8);
        assert_eq!(ans2, ErrType::PositiveOverflow((wp(78i8) + wp(122i8)).0));
        assert_eq!(ans3, ErrType::NegativeOverflow((wp(-78i8) + wp(-122i8)).0));
    }
}
