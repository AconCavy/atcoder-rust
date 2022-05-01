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
        n: f64,
    }

    const V: i32 = 206;
    let x = (n * 1.08) as i32;
    let mut answer = "so-so";
    if x < V {
        answer = "Yay!";
    }
    if x > V {
        answer = ":(";
    }
    println!("{}", answer);
}
