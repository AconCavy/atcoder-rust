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
        p: [(i32, i32); n],
        s: Chars,
    }

    const INF: i32 = 1e9 as i32;
    let mut map_l = HashMap::new();
    let mut map_r = HashMap::new();
    for ((x, y), c) in p.iter().zip(s.iter()) {
        if *c == 'L' {
            let v = map_l.entry(y).or_insert(-INF);
            *v = *x.max(v);
        } else {
            let v = map_r.entry(y).or_insert(INF);
            *v = *x.min(v);
        }
    }
    for (y, xl) in map_l {
        if let Some(xr) = map_r.get(y) {
            if *xr <= xl {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
