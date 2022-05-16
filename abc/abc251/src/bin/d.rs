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
        w: i32,
    }

    let mut answer = Vec::new();
    for i in 1..100 {
        answer.push(i);
        answer.push(i * 100);
        answer.push(i * 10000);
    }

    println!("{}", answer.len());
    println!("{}", answer.into_iter().join(" "));
}
