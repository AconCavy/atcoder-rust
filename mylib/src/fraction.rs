#![allow(dead_code)]

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Fraction {
    y: i64,
    x: i64,
}

impl Fraction {
    pub fn new(y: i64, x: i64) -> Self {
        fn gcd(a: i64, b: i64) -> i64 {
            return if b == 0 { a } else { gcd(b, a % b) };
        }

        let g = gcd(y, x);
        Self { y: y / g, x: x / g }
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y * other.x).cmp(&(self.x * other.y))
    }
}

impl PartialEq<Self> for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl PartialOrd<Self> for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
