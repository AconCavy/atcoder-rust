#![allow(dead_code)]

use crate::util::gcd;
use num_integer::Integer;

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Fraction<T: Copy + Integer> {
    y: T,
    x: T,
}

impl<T: Copy + Integer> Fraction<T> {
    pub fn new(y: T, x: T) -> Self {
        let g = gcd(y, x);
        Self { y: y / g, x: x / g }
    }
}

impl<T: Copy + Integer> Ord for Fraction<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y * other.x).cmp(&(self.x * other.y))
    }
}

impl<T: Copy + Integer> PartialEq<Self> for Fraction<T> {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl<T: Copy + Integer> PartialOrd<Self> for Fraction<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
