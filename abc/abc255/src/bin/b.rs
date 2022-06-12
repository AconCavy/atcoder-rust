#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use libm::sqrt;
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
        k: usize,
        a: [Usize1; k],
        p: [(i64, i64); n],
    }

    let mut dist_max = 0;
    for i in 0..n {
        let mut dist = 1e18 as i64;
        for &a in &a {
            let dx = p[i].0 - p[a].0;
            let dy = p[i].1 - p[a].1;
            dist = dist.min(dx * dx + dy * dy);
        }
        dist_max = max(dist_max, dist);
    }

    let answer = sqrt(dist_max as f64);
    println!("{}", answer);
}
