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
        h1: i32,
        h2: i32,
        h3: i32,
        w1: i32,
        w2: i32,
        w3: i32,
    }

    let mut answer = 0;
    for a in 1..=30 {
        for b in 1..=30 {
            for d in 1..=30 {
                for e in 1..=30 {
                    let c = h1 - a - b;
                    let f = h2 - d - e;
                    let g = w1 - a - d;
                    let h = w2 - b - e;
                    let i = h3 - g - h;
                    if i == w3 - c - f && c > 0 && f > 0 && g > 0 && h > 0 && i > 0 {
                        answer += 1;
                    }
                }
            }
        }
    }

    println!("{}", answer);
}
