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
        mut c: [i64; n],
    }

    const MOD: i64 = 1e9 as i64 + 7;
    c.sort();
    let answer = c
        .iter()
        .enumerate()
        .map(|(i, v)| max(v - i as i64, 0))
        .fold(1, |acc, x| (acc * x) % MOD);

    println!("{}", answer);
}
