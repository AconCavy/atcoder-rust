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

    let mut c = vec![0i64; 2e5 as usize + 1];
    for a in a {
        c[a] += 1;
    }

    let n64 = n as i64;
    let mut answer = n64 * (n64 - 1) * (n64 - 2) / 6;
    for c in c {
        answer -= c * (c - 1) * (c - 2) / 6;
        answer -= c * (c - 1) / 2 * (n64 - c);
    }

    println!("{}", answer);
}
