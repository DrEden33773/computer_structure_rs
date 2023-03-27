pub mod binary_opt {
    fn unsigned_add(a: u128, b: u128) -> u128 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let carry = a & b;
            a = a ^ b;
            b = carry << 1;
        }
        a
    }

    fn unsigned_mul(a: u128, b: u128) -> u128 {
        let mut a = a;
        let mut b = b;
        let mut result = 0;
        while b > 0 {
            if b & 1 == 1 {
                result = unsigned_add(result, a);
            }
            a <<= 1;
            b >>= 1;
        }
        result
    }

    pub fn mul(a: i128, b: i128) -> i128 {
        let bits_of_mul = unsigned_mul(a as u128, b as u128);
        let narrowed = bits_of_mul & u128::MAX;
        narrowed as i128
    }
}

pub mod booth {
    pub fn mul(multiplicand: i32, multiplier: i32) -> i64 {
        let mut accumulator: i64 = 0;
        let mut booth_register: i64 = multiplier as i64;
        let multiplicand = multiplicand as i64;
        let n = 32;

        for _ in 0..n {
            match booth_register & 3 {
                0b01 => accumulator += multiplicand,
                0b10 => accumulator -= multiplicand,
                _ => {}
            }
            accumulator >>= 1;
            booth_register >>= 1;
            booth_register |= (accumulator & 1) << (n - 1);
        }
        accumulator
    }
}

#[cfg(test)]
mod multiply {
    use super::*;

    #[test]
    fn test_bin_mul() {
        assert_eq!(binary_opt::mul(233, -23), 233 * -23);
        assert_eq!(binary_opt::mul(1024, -2244), 1024 * -2244);
    }

    // #[test]
    fn test_booth_mul() {
        assert_eq!(booth::mul(2, 23), 2 * 23);
    }
}
