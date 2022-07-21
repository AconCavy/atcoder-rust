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
        x: i64,
        t: [(i64, i64); n],
    }

    const INF: i64 = 7e18 as i64;
    let mut answer = INF;
    let mut cum = 0;
    for i in 0..n {
        cum += t[i].0;
        let sum = cum + t[i].1 * max(0, x - i as i64);
        answer = min(answer, sum);
        cum += t[i].1;
    }

    println!("{}", answer);
}
