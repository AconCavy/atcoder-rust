#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use num_integer::Integer;
use num_traits::Signed;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    let f = |x| x * (x + 1) / 2 >= n;
    let answer = binary_search(0, 1e5 as i64, f);
    println!("{}", answer);
}

pub fn binary_search<T: Copy + Integer + Signed, F: Fn(T) -> bool>(ng: T, ok: T, f: F) -> T {
    let mut ng = ng;
    let mut ok = ok;
    let two = T::one() + T::one();
    while (ok - ng).abs() > T::one() {
        let m = (ok + ng) / two;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}
