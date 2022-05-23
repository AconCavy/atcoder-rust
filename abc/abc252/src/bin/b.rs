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
        k: usize,
        a: [i32; n],
        b: [Usize1; k],
    }

    let max = a.iter().fold(0, |acc, &x| max(acc, x));
    let mut dislikes = vec![false; n];
    for (i, a) in a.into_iter().enumerate() {
        if a == max {
            dislikes[i] = true;
        }
    }
    let mut result = false;
    for b in b {
        result |= dislikes[b];
    }

    let answer = if result { "Yes" } else { "No" };
    println!("{}", answer);
}
