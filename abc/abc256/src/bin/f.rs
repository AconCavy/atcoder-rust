#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
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
        q: usize,
        mut a: [i64; n],
    }

    const MOD: i64 = 998_244_353;
    let mut ft1 = FenwickTree::new(n, 0, MOD);
    let mut ft2 = FenwickTree::new(n, 0, MOD);
    let mut ft3 = FenwickTree::new(n, 0, MOD);
    for i in 0..n {
        let x = i as i64;
        ft1.add(i, a[i]);
        ft2.add(i, a[i] * x % MOD);
        ft3.add(i, a[i] * x % MOD * x % MOD);
    }

    let i2 = power(2, MOD - 2, MOD);

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                i: Usize1,
                v: i64,
            }

            let x = i as i64;
            let d = (v - a[i] + MOD) % MOD;
            ft1.add(i, d);
            ft2.add(i, d * x % MOD);
            ft3.add(i, d * x % MOD * x % MOD);
            a[i] = v;
        } else {
            input! {
                i: Usize1,
            }

            let x = i as i64;
            let s1 = ft3.accum(i + 1);
            let s2 = ((x + 1) + (x + 2)) % MOD * ft2.accum(i + 1) % MOD;
            let s3 = ((x + 1) * (x + 2)) % MOD * ft1.accum(i + 1) % MOD;
            let answer = (((s1 - s2 + s3) * i2) % MOD + MOD) % MOD;
            println!("{}", answer);
        }
    }
}

fn power(v: i64, n: i64, m: i64) -> i64 {
    let mut result = 1;
    let mut n = n;
    let mut x = v % m;
    while n > 0 {
        if n & 1 == 1 {
            result *= x;
            result %= m;
        }

        n >>= 1;
        x *= x;
        x %= m;
    }

    result
}

pub struct FenwickTree {
    len: usize,
    data: Vec<i64>,
    e: i64,
    m: i64,
}

impl FenwickTree {
    pub fn new(len: usize, e: i64, m: i64) -> Self {
        Self {
            len,
            data: vec![e; len],
            e,
            m,
        }
    }

    pub fn add(&mut self, i: usize, v: i64) {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        while i as usize <= self.len {
            self.data[(i - 1) as usize] += v % self.m;
            self.data[(i - 1) as usize] %= self.m;
            i += i & -i;
        }
    }

    pub fn accum(&self, len: usize) -> i64 {
        assert!(len <= self.len);
        let mut len = len as i32;
        let mut sum = self.e;
        while len > 0 {
            sum += self.data[(len - 1) as usize];
            sum %= self.m;
            len -= len & -len;
        }

        if sum < 0 {
            sum + self.m
        } else {
            sum
        }
    }

    pub fn sum(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r <= self.len);
        let result = self.accum(r) - self.accum(l);
        if result < 0 {
            result + self.m
        } else {
            result
        }
    }
}
