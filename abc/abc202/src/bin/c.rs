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
        a: [usize; n],
        b: [usize; n],
        c: [Usize1; n],
    }

    let mut ca = vec![0; 1e5 as usize + 1];
    let mut cb = vec![0; 1e5 as usize + 1];
    for a in a {
        ca[a] += 1;
    }

    for c in c {
        cb[b[c]] += 1;
    }
    let answer = ca
        .into_iter()
        .zip(cb.into_iter())
        .fold(0i64, |acc, (a, b)| acc + a * b);
    println!("{}", answer);
}
