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
        m: usize,
        x: [i64; n],
    }

    let mut c = vec![0; n + 1];
    for _ in 0..m {
        input! {
            i: usize,
            y: i64,
        }

        c[i] = y;
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..=i {
            dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + x[i] + c[j + 1]);
            dp[i + 1][0] = max(dp[i + 1][0], dp[i][j]);
        }
    }

    let answer = dp[n].iter().copied().max().unwrap_or(0);
    println!("{}", answer);
}
