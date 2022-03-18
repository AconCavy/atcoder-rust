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
    }

    const MOD: i64 = 998244353;

    let mut edges = Vec::with_capacity(m);
    let mut deg = vec![n - 1; n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }

        edges.push((u, v));
        deg[u] += 1;
        deg[v] += 1;
    }

    let mut g: Vec<Vec<_>> = deg.into_iter().map(|x| Vec::with_capacity(x)).collect();
    for (u, v) in edges {
        g[u].push(v);
        g[v].push(u);
    }

    let mut dp = vec![vec![0; n]; k + 1];
    dp[0][0] = 1;
    for i in 0..k {
        let mut sum = 0;
        for u in 0..n {
            sum = (sum + dp[i][u]) % MOD;
        }
        for u in 0..n {
            dp[i + 1][u] = (sum - dp[i][u] + MOD) % MOD;
            for &v in &g[u] {
                dp[i + 1][u] = (dp[i + 1][u] - dp[i][v] + MOD) % MOD;
            }
        }
    }

    let answer = dp[k][0];
    println!("{}", answer);
}
