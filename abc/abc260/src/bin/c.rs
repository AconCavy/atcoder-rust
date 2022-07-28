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
        y: i64,
    }

    let mut dp_r = vec![0; n + 1];
    let mut dp_b = vec![0; n + 1];
    dp_r[n] = 1;

    for i in (2..=n).rev() {
        dp_r[i - 1] += dp_r[i];
        dp_b[i] += dp_r[i] * x;
        dp_r[i - 1] += dp_b[i];
        dp_b[i - 1] += dp_b[i] * y;
    }

    let answer = dp_b[1];
    println!("{}", answer);
}
