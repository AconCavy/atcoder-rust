#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use libm::{ceil, sqrt};
use num_integer::Roots;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64,
    }

    let sqd = x * x + y * y;
    let sqr = r * r;

    let answer = if sqd == sqr {
        1
    } else if sqd < sqr {
        2
    } else {
        let t = (sqd as f64).sqrt() / r as f64;
        t.ceil() as i64
    };

    println!("{}", answer);
}
