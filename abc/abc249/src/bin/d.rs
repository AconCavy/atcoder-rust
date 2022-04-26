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
    let max = 2e5 as usize;
    let mut count = vec![0i64; max + 1];
    for a in a {
        count[a] += 1;
    }

    let mut answer = 0;
    for r in 1..=max {
        for q in (1..=max).take_while(|q| q * r <= max) {
            let p = q * r;
            answer += count[p] * count[q] * count[r];
        }
    }

    println!("{}", answer);
}
