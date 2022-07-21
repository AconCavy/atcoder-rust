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
        _n: i32,
        m: i32,
        x: i32,
        t: i32,
        d: i32,
    }

    let answer = t - max(0, x - m) * d;
    println!("{}", answer);
}
