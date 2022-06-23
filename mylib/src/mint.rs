#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mint {
    val: i64,
}

impl Mint {
    pub const MOD: i64 = 998_244_353;

    pub fn new(val: i64) -> Self {
        let val = val % Mint::MOD;
        if val < 0 {
            Self {
                val: val + Mint::MOD,
            }
        } else {
            Self { val }
        }
    }

    pub fn pow(self, n: i64) -> Self {
        let mut val = 1;
        let mut n = n;
        let mut x = self.val;
        while n > 0 {
            if n & 1 == 1 {
                val = val * x % Mint::MOD;
            }

            n >>= 1;
            x = x * x % Mint::MOD;
        }

        Self::new(val)
    }

    pub fn inv(self) -> Self {
        if self.val == 0 {
            return self;
        }

        let mut s = Mint::MOD;
        let mut t = self.val;
        let mut m0 = 0;
        let mut m1 = 1;
        while t > 0 {
            let u = s / t;
            s -= t * u;
            m0 -= m1 * u;
            std::mem::swap(&mut s, &mut t);
            std::mem::swap(&mut m0, &mut m1);
        }

        if m0 < 0 {
            m0 += Mint::MOD / s;
        }

        Self::new(m0)
    }
}

impl Add for Mint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val + rhs.val)
    }
}

impl Sub for Mint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.val - rhs.val)
    }
}

impl Mul for Mint {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.val * rhs.val)
    }
}

impl Div for Mint {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.val * rhs.inv().val)
    }
}

impl AddAssign for Mint {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % Mint::MOD;
    }
}

impl SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (self.val - rhs.val + Mint::MOD) % Mint::MOD;
    }
}

impl MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = self.val * rhs.val % Mint::MOD;
    }
}

impl DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        self.val = self.val * rhs.inv().val % Mint::MOD;
    }
}

macro_rules! impl_from_integer_for_mint {
    ($($t:ident),*) => {
        $(
            impl From<$t> for Mint {
                fn from(v: $t) -> Self {
                    Self::new(v as i64)
                }
            }
        )*
    };
}

impl_from_integer_for_mint!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        assert_eq!(Mint::new(0).val, 0);
        assert_eq!(Mint::new(1).val, 1);
        assert_eq!(Mint::new(Mint::MOD + 1).val, 1);
        assert_eq!(Mint::new(-1).val, Mint::MOD - 1);
    }

    #[test]
    fn pow_test() {
        fn pow(v: i64, n: i64) -> i64 {
            Mint::new(v).pow(n).val
        }
        assert_eq!(pow(1, 0), 1);
        assert_eq!(pow(1, 1), 1);
        assert_eq!(pow(1, 2), 1);
        assert_eq!(pow(2, 2), 4);
        assert_eq!(pow(100_000, 100_000), 57_775_212);
    }

    #[test]
    fn inv_test() {
        fn inv(v: i64) -> i64 {
            Mint::new(v).inv().val
        }
        assert_eq!(inv(0), 0);
        assert_eq!(inv(1), 1);
        assert_eq!(inv(2), 499_122_177);
        assert_eq!(inv(7), 855_638_017);
        assert_eq!(inv(100_000), 179_514_282);
    }

    #[test]
    fn add_test() {
        fn add(lhs: i64, rhs: i64) -> i64 {
            (Mint::new(lhs) + Mint::new(rhs)).val
        }
        assert_eq!(add(1, 1), 2);
        assert_eq!(add(2, Mint::MOD - 1), 1);
    }

    #[test]
    fn sub_test() {
        fn sub(lhs: i64, rhs: i64) -> i64 {
            (Mint::new(lhs) - Mint::new(rhs)).val
        }
        assert_eq!(sub(2, 1), 1);
        assert_eq!(sub(1, 2), Mint::MOD - 1);
    }

    #[test]
    fn mul_test() {
        fn mul(lhs: i64, rhs: i64) -> i64 {
            (Mint::new(lhs) * Mint::new(rhs)).val
        }
        assert_eq!(mul(1, 1), 1);
        assert_eq!(mul(2, 2), 4);
        assert_eq!(mul(100_000, 100_000), 100_000 * 100_000 % Mint::MOD);
    }

    #[test]
    fn div_test() {
        fn div(lhs: i64, rhs: i64) -> i64 {
            (Mint::new(lhs) / Mint::new(rhs)).val
        }
        assert_eq!(div(0, 1), 0);
        assert_eq!(div(1, 1), 1);
        assert_eq!(div(2, 2), 1);
        assert_eq!(div(1, 7), 855_638_017);
    }

    #[test]
    fn add_assign_test() {
        fn add(lhs: i64, rhs: i64) -> i64 {
            let mut v = Mint::new(lhs);
            v += Mint::new(rhs);
            v.val
        }
        assert_eq!(add(1, 1), 2);
        assert_eq!(add(2, Mint::MOD - 1), 1);
    }

    #[test]
    fn sub_assign_test() {
        fn sub(lhs: i64, rhs: i64) -> i64 {
            let mut v = Mint::new(lhs);
            v -= Mint::new(rhs);
            v.val
        }
        assert_eq!(sub(2, 1), 1);
        assert_eq!(sub(1, 2), Mint::MOD - 1);
    }

    #[test]
    fn mul_assign_test() {
        fn mul(lhs: i64, rhs: i64) -> i64 {
            let mut v = Mint::new(lhs);
            v *= Mint::new(rhs);
            v.val
        }
        assert_eq!(mul(1, 1), 1);
        assert_eq!(mul(2, 2), 4);
        assert_eq!(mul(100_000, 100_000), 100_000 * 100_000 % Mint::MOD);
    }

    #[test]
    fn div_assign_test() {
        fn div(lhs: i64, rhs: i64) -> i64 {
            let mut v = Mint::new(lhs);
            v /= Mint::new(rhs);
            v.val
        }
        assert_eq!(div(0, 1), 0);
        assert_eq!(div(1, 1), 1);
        assert_eq!(div(2, 2), 1);
        assert_eq!(div(1, 7), 855_638_017);
    }
}
