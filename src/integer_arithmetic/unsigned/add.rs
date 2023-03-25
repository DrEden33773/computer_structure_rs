//! Notifications
//!
//! u8::MAX = 2^8-1 = 255
//!
//! u8::MIN = 0

#![allow(dead_code)]

use std::num::Wrapping as wp;

pub fn u8_add_ok(lhs: u8, rhs: u8) -> bool {
    let sum = wp(lhs) + wp(rhs);
    return sum >= wp(lhs);
}

#[cfg(test)]
mod u8_add {
    use super::*;

    #[test]
    fn test_u8_add_ok() {
        let ans1 = u8_add_ok(3, 8);
        let ans2 = u8_add_ok(233, 112);
        assert_eq!((ans1, ans2), (true, false))
    }
}
