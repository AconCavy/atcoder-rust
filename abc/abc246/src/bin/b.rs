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
        a: f64,
        b: f64,
    }

    let d = (a * a + b * b).sqrt();
    let x = a / d;
    let y = b / d;
    println!("{} {}", x, y);
}
