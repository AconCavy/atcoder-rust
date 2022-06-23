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
        range: [(usize, usize); n],
    }

    let mut cum = vec![0; 2e5 as usize + 10];
    for (l, r) in range {
        cum[l] += 1;
        cum[r] -= 1;
    }

    for i in 1..cum.len() {
        cum[i] += cum[i - 1];
    }

    let mut l = 1;
    let mut r = 1;
    while l < cum.len() {
        if cum[l] != 0 {
            r = l;
            while r < cum.len() && cum[r] > 0 {
                r += 1;
            }

            println!("{} {}", l, r);
            l = r;
        }

        l += 1;
    }
}
