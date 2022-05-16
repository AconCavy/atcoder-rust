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
        a: [i64; n],
    }

    const INF: i64 = 1e18 as i64;
    let mut answer = INF;
    for k in 0..2 {
        let mut dp = vec![vec![INF; 2]; n + 1];
        if k == 0 {
            dp[1][0] = 0;
            dp[1][1] = INF;
        } else {
            dp[1][0] = INF;
            dp[1][1] = a[0];
        }

        for i in 1..n {
            dp[i + 1][0] = dp[i][1];
            dp[i + 1][1] = min(dp[i][0], dp[i][1]) + a[i];
        }

        let sum = if k == 0 {
            dp[n][1]
        } else {
            min(dp[n][0], dp[n][1])
        };

        answer = min(answer, sum);
    }

    println!("{}", answer);
}
