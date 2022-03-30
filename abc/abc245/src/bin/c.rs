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
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut dp1 = HashSet::new();
    dp1.insert(a[0]);
    dp1.insert(b[0]);

    for i in 1..n {
        let mut dp2 = HashSet::new();
        for x in &dp1 {
            if (a[i] - x).abs() <= k {
                dp2.insert(a[i]);
            }
            if (b[i] - x).abs() <= k {
                dp2.insert(b[i]);
            }
        }

        dp1 = dp2;
    }

    let answer = dp1.len() > 0;
    println!("{}", if answer { "Yes" } else { "No" });
}
