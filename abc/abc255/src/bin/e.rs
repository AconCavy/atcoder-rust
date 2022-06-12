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
        s: [i64; n - 1],
        x: [i64; m],
    }

    let mut b = vec![0; n];
    for i in 0..n - 1 {
        b[i + 1] = s[i] - b[i];
    }

    let mut map = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            let c = if i % 2 == 0 { x[j] - b[i] } else { b[i] - x[j] };
            *map.entry(c).or_insert(0) += 1;
        }
    }

    let answer = map.values().fold(0, |acc, &x| max(acc, x));
    println!("{}", answer);
}
