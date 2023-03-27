pub mod binary_opt {
    pub fn add(a: u128, b: u128) -> u128 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let carry = a & b;
            a = a ^ b;
            b = carry << 1;
        }
        a
    }

    pub fn mul(a: u128, b: u128) -> u128 {
        let mut a = a;
        let mut b = b;
        let mut result = 0;
        while b > 0 {
            if b & 1 == 1 {
                result = add(result, a);
            }
            a <<= 1;
            b >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod multiply {
    use super::*;

    #[test]
    fn test_bin_mul() {
        assert_eq!(binary_opt::mul(233, 23), 233 * 23);
        assert_eq!(binary_opt::mul(1024, 2244), 1024 * 2244);
    }
}
