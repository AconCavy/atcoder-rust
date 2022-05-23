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

    let mut g = vec![Vec::new(); n];
    let mut e = HashMap::new();
    for i in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: i64,
        }

        g[a].push((b, c));
        g[b].push((a, c));
        e.insert((a, b), i + 1);
        e.insert((b, a), i + 1);
    }

    const INF: i64 = 1e18 as i64;
    let mut costs = vec![INF; n];
    costs[0] = 0;
    let mut prev = vec![0; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));

    while let Some((Reverse(uc), u)) = heap.pop() {
        if uc > costs[u] {
            continue;
        }

        for &(v, vc) in &g[u] {
            let c = uc + vc;
            if c < costs[v] {
                costs[v] = c;
                prev[v] = u;
                heap.push((Reverse(c), v));
            }
        }
    }

    let mut result = Vec::with_capacity(n - 1);
    for v in 1..n {
        let u = prev[v];
        let idx = *e.entry((u, v)).or_default();
        result.push(idx);
    }

    let answer = result.into_iter().join(" ");
    println!("{}", answer);
}
