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
        a: [usize; n],
    }

    let a = a.into_iter().map(|x| x % 200).collect::<Vec<_>>();
    let mut count = vec![0i64; 200];
    for &a in &a {
        count[a] += 1;
    }

    let mut answer = 0;
    for c in count {
        answer += c * (c - 1) / 2;
    }

    println!("{}", answer);
}
