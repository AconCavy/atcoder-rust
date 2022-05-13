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
        s: Chars,
    }

    let mut answer = Vec::with_capacity(s.len());
    for c in s.into_iter().rev() {
        let x = match c {
            '6' => '9',
            '9' => '6',
            _ => c,
        };

        answer.push(x);
    }

    println!("{}", answer.iter().join(""));
}
