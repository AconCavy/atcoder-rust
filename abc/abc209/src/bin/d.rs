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
        q: usize,
    }

    let mut g = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }

        g[a].push(b);
        g[b].push(a);
    }

    let mut color = vec![0; n];
    let mut queue = VecDeque::new();
    queue.push_back((0, n));
    while let Some((u, p)) = queue.pop_front() {
        for &v in &g[u] {
            if v != p {
                color[v] = color[u] ^ 1;
                queue.push_back((v, u));
            }
        }
    }

    for _ in 0..q {
        input! {
            c: Usize1,
            d: Usize1,
        }

        let answer = if color[c] == color[d] { "Town" } else { "Road" };
        println!("{}", answer);
    }
}
