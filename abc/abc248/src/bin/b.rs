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
        a: u64,
        b: u64,
        k: u64,
    }

    let mut curr = a;
    let mut answer = 0;
    while curr < b {
        curr *= k;
        answer += 1;
    }

    println!("{}", answer);
}
