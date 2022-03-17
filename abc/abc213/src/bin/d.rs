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
    }

    let mut edges = Vec::with_capacity(n - 1);
    let mut deg = vec![0; n];
    for _ in 0..(n - 1) {
        input! {
            a: Usize1,
            b: Usize1,
        }
        edges.push((a, b));
        deg[a] += 1;
        deg[b] += 1;
    }

    let mut g: Vec<_> = deg.iter().map(|x| Vec::with_capacity(*x)).collect();
    for (a, b) in &edges {
        g[*a].push(*b);
        g[*b].push(*a);
    }

    for u in g.iter_mut() {
        u.sort_unstable();
    }

    let mut used = vec![false; n];
    let mut answer = Vec::new();
    fn dfs(answer: &mut Vec<usize>, used: &mut Vec<bool>, g: &Vec<Vec<usize>>, u: usize) {
        answer.push(u);
        used[u] = true;
        for &v in &g[u] {
            if !used[v] {
                dfs(answer, used, g, v);
                answer.push(u);
            }
        }
    }

    dfs(&mut answer, &mut used, &g, 0);
    println!("{}", answer.into_iter().map(|x| x + 1).join(" "));
}
