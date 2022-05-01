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
        a: [i32; 3],
    }

    let sum = a.iter().fold(0, |acc, x| acc + x);
    let min = a.iter().fold(1000, |acc, x| acc.min(*x));
    let answer = sum - min;
    println!("{}", answer);
}
