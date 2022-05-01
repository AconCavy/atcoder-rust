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
        a: [i64; n],
    }

    let sum = a.iter().fold(0, |acc, x| acc + x) - (n as i64 / 2);
    let answer = if x >= sum { "Yes" } else { "No" };
    println!("{}", answer);
}
