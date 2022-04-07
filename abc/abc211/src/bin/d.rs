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
    const INF: i64 = 1e18 as i64;

    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }

        g[a].push(b);
        g[b].push(a);
    }

    let mut dp = vec![0; n];
    let mut costs = vec![-1; n];

    dp[0] = 1;
    costs[0] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in &g[u] {
            if costs[v] != -1 {
                if costs[v] == costs[u] + 1 {
                    dp[v] += dp[u];
                    dp[v] %= MOD;
                }
                continue;
            }

            costs[v] = costs[u] + 1;
            dp[v] += dp[u];
            dp[v] %= MOD;
            queue.push_back(v);
        }
    }

    let answer = dp[n - 1];
    println!("{}", answer);
}
