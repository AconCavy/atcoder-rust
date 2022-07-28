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
        l1: i32,
        r1: i32,
        l2: i32,
        r2: i32,
    }

    let answer = max(0, min(r1, r2) - max(l1, l2));
    println!("{}", answer);
}
