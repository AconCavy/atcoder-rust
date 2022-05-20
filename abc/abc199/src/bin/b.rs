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
        a: [i32; n],
        b: [i32; n],
    }

    let a = a.into_iter().fold(0, max);
    let b = b.into_iter().fold(1001, min);
    let answer = max(0, b - a + 1);
    println!("{}", answer);
}
