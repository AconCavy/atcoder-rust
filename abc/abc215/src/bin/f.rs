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
        mut p: [(i32, i32); n],
    }

    p.sort_by_key(|x| x.0);

    const INF: i32 = 1e9 as i32;

    let f = |k: i32| -> bool {
        let mut min_y = INF;
        let mut max_y = 0;
        let mut j = 0;
        for (i, (x, y)) in p.iter().enumerate() {
            while j < i && *x - p[j].0 >= k {
                min_y = min_y.min(p[j].1);
                max_y = max_y.max(p[j].1);
                j += 1;
            }

            if *y - min_y >= k || max_y - *y >= k {
                return true;
            }
        }

        false
    };

    let answer = binary_search(1e9 as i32 + 1, 0, f);
    println!("{}", answer);
}

fn binary_search<F: Fn(i32) -> bool>(ng: i32, ok: i32, f: F) -> i32 {
    let mut ng = ng;
    let mut ok = ok;
    while (ok - ng).abs() > 1 {
        let m = (ok + ng) / 2;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}
