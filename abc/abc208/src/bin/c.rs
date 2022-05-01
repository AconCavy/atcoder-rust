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
        k: i64,
        a: [i64; n],
    }

    let mut answer = vec![k / n as i64; n];
    let mut idx: Vec<_> = a.iter().enumerate().map(|(i, a)| (i, a)).collect();
    idx.sort_by_key(|x| x.1);
    for &i in idx.iter().map(|(i, _)| i).take((k % n as i64) as usize) {
        answer[i] += 1;
    }

    println!("{}", answer.into_iter().join("\n"));
}
