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
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32,
        x: i32,
    }

    let g = |a, b, c, x| (x / (a + c) * a + min(x % (a + c), a)) * b;
    let answer = match (g(a, b, c, x), g(d, e, f, x)) {
        (t, a) if t > a => "Takahashi",
        (t, a) if t < a => "Aoki",
        _ => "Draw",
    };
    println!("{}", answer);
}
