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
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        e: [(Usize1, Usize1); m],
    }

    const MOD: i64 = 998244353;
    let mut g = vec![Vec::new(); n];
    for &(u, v) in &e {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dp = vec![vec![(0, 0); n]; k + 1];
    dp[0][s] = (1, 0);

    for k in 0..k {
        for u in 0..n {
            for &v in &g[u] {
                if v == x {
                    dp[k + 1][v].0 += dp[k][u].1;
                    dp[k + 1][v].0 %= MOD;
                    dp[k + 1][v].1 += dp[k][u].0;
                    dp[k + 1][v].1 %= MOD;
                } else {
                    dp[k + 1][v].0 += dp[k][u].0;
                    dp[k + 1][v].0 %= MOD;
                    dp[k + 1][v].1 += dp[k][u].1;
                    dp[k + 1][v].1 %= MOD;
                }
            }
        }
    }

    let answer = dp[k][t].0;
    println!("{}", answer);
}
