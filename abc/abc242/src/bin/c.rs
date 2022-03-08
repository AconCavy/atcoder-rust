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
    }

    const M: i64 = 998244353;

    let mut dp = vec![vec![0i64; 11]; n + 1];
    for i in 1..=9 {
        dp[1][i] = 1;
    }

    for i in 1..n {
        for j in 1..=9 {
            dp[i + 1][j - 1] += dp[i][j];
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j + 1] += dp[i][j];

            dp[i + 1][j - 1] %= M;
            dp[i + 1][j] %= M;
            dp[i + 1][j + 1] %= M;
        }
    }

    let answer = dp[n].iter().skip(1).take(9).fold(0i64, |s, v| (s + v) % M);
    println!("{}", answer);
}
