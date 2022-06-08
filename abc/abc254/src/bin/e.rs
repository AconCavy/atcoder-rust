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
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }

        g[a].push(b);
        g[b].push(a);
    }

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            x: Usize1,
            k: usize,
        }

        let mut queue = VecDeque::new();
        queue.push_back((x, 0));
        let mut used = HashSet::new();
        used.insert(x);
        while let Some((u, t)) = queue.pop_front() {
            if t == k {
                continue;
            }

            for v in &g[u] {
                if !used.contains(v) {
                    queue.push_back((*v, t + 1));
                    used.insert(*v);
                }
            }
        }

        let answer = used.into_iter().fold(0, |acc, u| acc + u + 1);
        println!("{}", answer);
    }
}
