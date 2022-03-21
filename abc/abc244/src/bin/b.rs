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
        _: usize,
        t: Chars,
    }

    let mut x = 0;
    let mut y = 0;
    let mut curr = 0;
    let d4 = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    for c in t {
        if c == 'S' {
            let (dx, dy) = d4[curr];
            x += dx;
            y += dy;
        } else {
            curr += 1;
            curr %= 4;
        }
    }

    println!("{} {}", x, y);
}
