#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use nalgebra::DimAdd;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [(i64, i64); n],
    }

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut answer = 0;
    for (i, &(x1, y1)) in p.iter().enumerate() {
        let mut map = HashMap::new();
        for &(x2, y2) in p.iter().skip(i + 1) {
            let a = Fraction::new(y2 - y1, x2 - x1);
            *map.entry(a).or_insert(0) += 1;
        }

        answer += map.values().filter(|&c| *c == k - 1).count();
    }

    println!("{}", answer);
}

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
