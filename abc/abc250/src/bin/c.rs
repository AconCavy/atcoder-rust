#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::*;
use std::{io, mem};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut a = vec![0; n];
    let mut idx = vec![0; n + 1];
    for i in 0..n {
        a[i] = i + 1;
        idx[i + 1] = i;
    }

    for _ in 0..q {
        input! {
            x: usize,
        }

        let i = idx[x];
        let j = if i == n - 1 { i - 1 } else { i + 1 };
        let y = a[j];
        a.swap(i, j);
        idx.swap(x, y);
    }

    println!("{}", a.into_iter().join(" "));
}
