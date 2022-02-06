use proconio::*;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut p: Vec<(Fraction, Fraction)> = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
        }

        p.push((Fraction::new(y - 1, x), Fraction::new(y, x - 1)));
    }

    p.sort_by_key(|x| x.1);
    let mut answer = 0;
    let mut rr = Fraction::new(-1, 1);
    for (l, r) in p {
        if rr <= l {
            answer += 1;
            rr = r;
        }
    }

    println!("{}", answer);
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
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
