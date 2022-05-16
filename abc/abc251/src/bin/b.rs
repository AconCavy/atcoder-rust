#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use nalgebra::DimNameAdd;
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
        w: i64,
        a: [i64; n],
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(a[i]);
        for j in i + 1..n {
            set.insert(a[i] + a[j]);
            for k in j + 1..n {
                set.insert(a[i] + a[j] + a[k]);
            }
        }
    }

    let answer = set.into_iter().filter(|&x| x <= w).count();
    println!("{}", answer);
}
