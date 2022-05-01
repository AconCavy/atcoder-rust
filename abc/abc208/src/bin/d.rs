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
    }

    const INF: i64 = 1e18 as i64;
    let mut g = vec![vec![INF; n]; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: i64,
        }

        g[a][b] = c;
    }

    let mut answer = 0;
    for k in 0..n {
        for s in 0..n {
            for t in 0..n {
                g[s][t] = min(g[s][t], g[s][k] + g[k][t]);
                answer += if s == t || g[s][t] == INF { 0 } else { g[s][t] };
            }
        }
    }

    println!("{}", answer);
}
