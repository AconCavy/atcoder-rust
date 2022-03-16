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
        s: [i64; n],
        t: [i64; n],
    }

    let mut g = t.clone();
    for i in 0..(n * 2) {
        let curr = i % n;
        let next = (i + 1) % n;
        g[next] = g[next].min(g[curr] + s[curr]);
    }

    println!("{}", g.iter().join("\n"));
}
