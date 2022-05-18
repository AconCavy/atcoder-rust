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
        k: usize
    }

    let mut answer = n;
    for _ in 0..k {
        if answer % 200 == 0 {
            answer /= 200;
        } else {
            answer *= 1000;
            answer += 200;
        }
    }

    println!("{}", answer);
}
