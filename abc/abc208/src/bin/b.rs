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
        mut p: i64,
    }

    let mut c = vec![0; 11];
    c[0] = 1;
    for i in 1..=10 {
        c[i] = c[i - 1] * i as i64;
    }

    let mut answer = 0;
    for i in (1..=10).rev() {
        answer += p / c[i];
        p %= c[i];
    }

    println!("{}", answer);
}
