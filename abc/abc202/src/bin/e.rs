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
        p: [Usize1; n - 1],
        q: usize,
    }

    let mut tree = vec![Vec::new(); n];
    for i in 0..n - 1 {
        let u = i + 1;
        let p = p[i];
        tree[p].push(u);
    }

    let mut t_in = vec![0; n];
    let mut t_out = vec![0; n];
    let mut t = 0;
    let mut depth = vec![0; n];

    fn dfs(
        tree: &[Vec<usize>],
        u: usize,
        t_in: &mut [usize],
        t_out: &mut [usize],
        t: &mut usize,
        depth: &mut [usize],
    ) {
        t_in[u] = *t;
        *t += 1;

        for &v in &tree[u] {
            depth[v] = depth[u] + 1;
            dfs(tree, v, t_in, t_out, t, depth);
        }

        t_out[u] = *t;
        *t += 1;
    }

    dfs(&tree, 0, &mut t_in, &mut t_out, &mut t, &mut depth);

    let mut depth_t = vec![Vec::new(); n];
    for u in 0..n {
        depth_t[depth[u]].push(t_in[u]);
    }

    for d in depth_t.iter_mut() {
        d.sort();
    }

    for _ in 0..q {
        input! {
            u: Usize1,
            d: usize,
        }

        let r = lower_bound(&depth_t[d], t_out[u]);
        let l = lower_bound(&depth_t[d], t_in[u]);
        let answer = r - l;
        println!("{}", answer);
    }
}

pub fn lower_bound<T: Ord>(v: &[T], key: T) -> usize {
    let mut l = -1;
    let mut r = v.len() as isize;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if v[m as usize] >= key {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}
