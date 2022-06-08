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
        mut a: [i32; n],
    }

    let mut b = vec![Vec::new(); k];
    for i in 0..n {
        b[i % k].push(a[i]);
    }

    for b in b.iter_mut() {
        b.sort();
    }

    let mut ok = true;
    a.sort();
    for i in 0..n {
        ok &= a[i] == b[i % k][i / k];
    }

    let answer = if ok { "Yes" } else { "No" };
    print!("{}", answer);
}
