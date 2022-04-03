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
        n: i64,
    }

    let f = |a: i64, b: i64| (a * a * a) + (a * a * b) + (a * b * b) + (b * b * b);
    const INF: i64 = 1e6 as i64;
    let mut answer = INF * INF * INF;
    let mut a = 0;
    let mut b = INF;
    while a <= b {
        while b >= a && f(a, b) >= n {
            answer = answer.min(f(a, b));
            b -= 1;
        }

        a += 1;
    }

    println!("{}", answer);
}
