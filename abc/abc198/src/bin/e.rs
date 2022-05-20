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
        c: [usize; n],
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

    let mut used = vec![0; 1e5 as usize + 1];
    let mut valid = vec![true; n];
    fn dfs(
        u: usize,
        p: usize,
        g: &[Vec<usize>],
        c: &[usize],
        used: &mut [i32],
        valid: &mut [bool],
    ) {
        if used[c[u]] > 0 {
            valid[u] = false;
        }

        used[c[u]] += 1;
        for &v in &g[u] {
            if v != p {
                dfs(v, u, g, c, used, valid);
            }
        }
        used[c[u]] -= 1;
    }

    dfs(0, n, &g, &c, &mut used, &mut valid);
    let answer = valid
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x)
        .map(|(i, _)| i + 1)
        .join("\n");
    println!("{}", answer);
}
