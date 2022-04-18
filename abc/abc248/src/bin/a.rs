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
        s: Bytes,
    }

    let answer = 9 * 10 / 2 - s.into_iter().map(|c| c - b'0').fold(0, |acc, x| acc + x);
    println!("{}", answer);
}
