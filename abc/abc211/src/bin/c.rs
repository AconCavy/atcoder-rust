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
    const MOD: i64 = 1e9 as i64 + 7;

    input! {
        s: Chars,
    }
    let mut dp = vec![0i64; 8];
    for c in s {
        let idx = match c {
            'c' => 0,
            'h' => 1,
            'o' => 2,
            'k' => 3,
            'u' => 4,
            'd' => 5,
            'a' => 6,
            'i' => 7,
            _ => 10,
        };

        if idx == 0 {
            dp[idx] += 1;
            dp[idx] %= MOD;
        } else if idx != 10 {
            dp[idx] += dp[idx - 1];
            dp[idx] %= MOD;
        }
    }

    let answer = dp[7];
    println!("{}", answer);
}
