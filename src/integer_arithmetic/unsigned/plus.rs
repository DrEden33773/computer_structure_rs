#![allow(dead_code)]

use std::num::Wrapping as wp;

pub fn u8_add_ok(a: u8, b: u8) -> bool {
    let sum = wp(a) + wp(b);
    return sum >= wp(a);
}

#[cfg(test)]
mod u8_plus {
    use super::*;

    #[test]
    fn test_u8_add_ok() {
        let ans1 = u8_add_ok(3, 8);
        let ans2 = u8_add_ok(233, 112);
        assert_eq!((ans1, ans2), (true, false))
    }
}
