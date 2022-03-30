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
    let mut deg = vec![0; n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }

        g[v].push(u);
        deg[u] += 1;
    }

    let mut answer = n;
    let mut queue = VecDeque::new();
    for i in 0..n {
        if deg[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(u) = queue.pop_front() {
        answer -= 1;
        for &v in &g[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                queue.push_back(v);
            }
        }
    }

    println!("{}", answer);
}
