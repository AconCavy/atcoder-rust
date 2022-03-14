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
        a: [i32; n],
        b: [i32; n],
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..n {
        for j in 0..n {
            if a[i] == b[j] {
                if i == j {
                    ans1 += 1;
                } else {
                    ans2 += 1;
                }
            }
        }
    }

    println!("{}\n{}", ans1, ans2);
}
