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

    let mut used = vec![false; n + 1];
    for a in a {
        used[a] = true;
    }

    let result = used.into_iter().skip(1).all(|x| x);
    let answer = if result { "Yes" } else { "No" };
    println!("{}", answer);
}
