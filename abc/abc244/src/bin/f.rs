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
        e: [(Usize1, Usize1); m],
    }

    let mut g = vec![Vec::new(); n];
    for &(u, v) in &e {
        g[u].push(v);
        g[v].push(u);
    }
    const INF: i32 = 1e9 as i32;
    let mut dp = vec![vec![INF; n]; 1 << n];
    let mut queue = VecDeque::new();
    for i in 0..n {
        dp[0][i] = 0;
        dp[1 << i][i] = 1;

        queue.push_back((1 << i, i, 1));
        while let Some((us, u, ul)) = queue.pop_front() {
            for &v in &g[u] {
                let vs = us ^ (1 << v);
                let vl = ul + 1;
                if vl < dp[vs][v] {
                    dp[vs][v] = vl;
                    queue.push_back((vs, v, vl));
                }
            }
        }
    }

    let mut answer = 0;
    for i in 0..(1 << n) {
        let mut min = INF;
        for &v in &dp[i] {
            min = min.min(v);
        }

        answer += min;
    }

    println!("{}", answer);
}
