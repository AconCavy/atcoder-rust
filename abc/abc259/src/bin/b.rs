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
        d: f64,
    }

    let rad = std::f64::consts::PI * d / 180.0;
    let sin = f64::sin(rad);
    let cos = f64::cos(rad);
    let x = cos * a - sin * b;
    let y = sin * a + cos * b;
    println!("{} {}", x, y);
}
