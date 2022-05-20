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

    let mut used = vec![false; n];
    let mut route = Vec::with_capacity(n);
    let mut color = vec![-1; n];

    fn make_route(u: usize, g: &[Vec<usize>], route: &mut Vec<usize>, used: &mut [bool]) {
        used[u] = true;
        route.push(u);
        for &v in &g[u] {
            if !used[v] {
                make_route(v, g, route, used);
            }
        }
    }

    fn calc(idx: usize, g: &[Vec<usize>], route: &[usize], color: &mut [i32]) -> i64 {
        let u = route[idx];
        for &v in &g[u] {
            if color[u] == color[v] {
                return 0;
            }
        }

        if idx == route.len() - 1 {
            return 1;
        }

        let mut result = 0;
        let x = route[idx + 1];
        for c in 0..3 {
            color[x] = c;
            result += calc(idx + 1, g, route, color);
            color[x] = -1;
        }

        result
    }

    let mut answer = 1;
    for u in 0..n {
        if !used[u] {
            route.clear();
            make_route(u, &g, &mut route, &mut used);
            color[u] = 0;
            answer *= calc(0, &g, &route, &mut color) * 3;
        }
    }

    println!("{}", answer);
}
