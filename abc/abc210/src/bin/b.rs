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
        s: Chars,
    }

    for (i, c) in s.into_iter().enumerate() {
        if c == '1' {
            let answer = if i % 2 == 0 { "Takahashi" } else { "Aoki" };
            println!("{}", answer);
            return;
        }
    }
}
