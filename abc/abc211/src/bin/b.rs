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
    let mut set = HashSet::new();
    for _ in 0..4 {
        input! {
            s: String,
        }

        set.insert(s);
    }

    let answer = set.len() == 4;
    println!("{}", if answer { "Yes" } else { "No" });
}
