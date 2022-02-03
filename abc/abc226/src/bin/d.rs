use num_integer::gcd;
use proconio::*;
use std::cmp::Ordering;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(i64, i64); n]
    }

    let mut set: HashSet<Fraction> = HashSet::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dy = p[i].1 - p[j].1;
            let dx = p[i].0 - p[j].0;
            set.insert(Fraction::new(dy, dx));
            set.insert(Fraction::new(-dy, -dx));
        }
    }

    let answer = set.len() * 2;
    println!("{}", answer);
}

#[derive(Eq, Hash)]
struct Fraction {
    y: i64,
    x: i64,
}

impl Fraction {
    fn new(y: i64, x: i64) -> Self {
        fn gcd(a: i64, b: i64) -> i64 {
            return if b == 0 { a } else { gcd(b, a % b) };
        }

        let g = gcd(y, x);
        Self { y: y / g, x: x / g }
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y * other.x).cmp(&(self.x * other.y))
    }
}

impl PartialEq<Self> for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl PartialOrd<Self> for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
