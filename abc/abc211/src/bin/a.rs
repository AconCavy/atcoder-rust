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
        a: f64,
        b: f64,
    }

    let c = |a: f64, b: f64| (a - b) / 3.0 + b;
    let answer = c(a, b);
    println!("{}", answer);
}
