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
        e: [(Usize1, Usize1, i64); m],
    }

    const INF: i64 = 1e18 as i64;
    let mut g = vec![vec![INF; n]; n];
    for (u, v, c) in &e {
        g[*u][*v] = *c;
        g[*v][*u] = *c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }

    let mut answer = 0;
    for (u, v, c) in &e {
        for k in 0..n {
            if g[*u][k] + g[k][*v] <= *c {
                answer += 1;
                break;
            }
        }
    }
    println!("{}", answer);
}
