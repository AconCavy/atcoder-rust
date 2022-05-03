#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use num_integer::Roots;
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

    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: i64,
            d: i64,
        }

        g[a].push((b, c, d));
        g[b].push((a, c, d));
    }

    let f = |t, c, d| t + c + d / (t + 1);
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    const INF: i64 = 1e18 as i64;
    let mut result = vec![INF; n];
    result[0] = 0;

    while let Some((Reverse(t), u)) = heap.pop() {
        if result[u] < t {
            continue;
        }
        for &(v, c, d) in &g[u] {
            let sq = d.sqrt();
            let nt = (max(t, sq - 2)..=max(t, sq + 2))
                .map(|x| f(x, c, d))
                .fold(INF, min);

            if nt < result[v] {
                result[v] = nt;
                heap.push((Reverse(nt), v));
            }
        }
    }

    let answer = if result[n - 1] == INF {
        -1
    } else {
        result[n - 1]
    };
    println!("{}", answer);
}
