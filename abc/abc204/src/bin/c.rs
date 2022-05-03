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
    }

    fn dfs(g: &[Vec<usize>], used: &mut Vec<bool>, u: usize) {
        if used[u] {
            return;
        }
        used[u] = true;
        for &v in &g[u] {
            dfs(g, used, v)
        }
    }

    let mut answer = 0;
    for i in 0..n {
        let mut used = vec![false; n];
        dfs(&g, &mut used, i);
        answer += used.into_iter().filter(|&x| x).count();
    }

    println!("{}", answer);
}
