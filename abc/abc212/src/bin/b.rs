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
        x: Bytes,
    }

    let x: Vec<_> = x.iter().map(|x| x - b'0').collect();
    let mut count = 0;
    for i in 0..3 {
        count += if (x[i] + 1) % 10 == x[i + 1] { 1 } else { 0 };
    }

    if count == 3 || x[0] == x[1] && x[0] == x[2] && x[0] == x[3] {
        println!("Weak");
    } else {
        println!("Strong");
    }
}
