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
    let mut x = 0;
    let mut y = 0;
    for _ in 0..3 {
        input! {
            xi: i32,
            yi: i32,
        }

        x ^= xi;
        y ^= yi;
    }

    println!("{} {}", x, y);
}
