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
        mut k: i64,
        x: i64,
        mut a: [i64; n],
    }

    for i in 0..n {
        let kk = (a[i] / x).min(k);
        a[i] -= kk * x;
        k -= kk;
    }

    a.sort_unstable();
    a.reverse();

    for i in 0..n.min(k as usize) {
        a[i] = 0;
    }

    let answer = a.iter().fold(0, |acc, x| acc + x);
    println!("{}", answer);
}
