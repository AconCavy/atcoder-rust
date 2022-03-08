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
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let answer = if x <= a {
        1.0
    } else if a < x && x <= b {
        c as f64 / (b - a) as f64
    } else {
        0.0
    };

    println!("{}", answer);
}
