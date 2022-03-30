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
        m: usize,
        a: [i32; n + 1],
        mut c: [i32; n + m + 1],
    }

    let mut b = vec![0; m + 1];

    for bi in (0..=m).rev() {
        b[bi] = c[n + bi] / a[n];
        for ai in 0..=n {
            c[ai + bi] -= a[ai] * b[bi];
        }
    }

    println!("{}", b.iter().join(" "));
}
