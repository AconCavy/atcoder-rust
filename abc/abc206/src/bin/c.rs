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
        a: [i32; n],
    }

    let mut map = HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0) += 1;
    }

    let answer = map.values().map(|c| c * (n - c)).fold(0, |acc, x| acc + x) / 2;
    println!("{}", answer);
}
