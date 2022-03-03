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
        mut n: i64,
    }

    let mut answer = Vec::with_capacity(120);
    while n > 0 {
        if n & 1 == 1 {
            answer.push('A');
        }

        n >>= 1;
        if n > 0 {
            answer.push('B');
        }
    }

    answer.reverse();
    println!("{}", answer.into_iter().join(""));
}
