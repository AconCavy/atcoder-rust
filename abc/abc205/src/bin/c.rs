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
        a: i64,
        b: i64,
        mut c: u32,
    }

    if c > 2 {
        c = if c % 2 == 0 { 2 } else { 1 };
    }

    let answer = match (a.pow(c), b.pow(c)) {
        (a, b) if a > b => '>',
        (a, b) if a < b => '<',
        _ => '=',
    };
    println!("{}", answer);
}
