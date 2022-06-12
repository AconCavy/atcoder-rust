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
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }

    let f = |v: i64| (x - (a + d * v)).abs();
    let answer = if d == 0 {
        f(0)
    } else {
        let div = max(0, min(n - 2, (x - a) / d));
        min(f(div), f(div + 1))
    };

    println!("{}", answer);
}
