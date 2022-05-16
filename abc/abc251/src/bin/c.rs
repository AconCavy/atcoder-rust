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
        n: usize,
        sub: [(String, i32); n],
    }

    let mut set = HashSet::new();
    let mut original = Vec::new();
    for (s, t) in sub.iter() {
        let s = s.clone();
        if !set.contains(&s) {
            set.insert(s.clone());
            original.push((s, *t));
        }
    }

    let mut max = -1;
    let mut result = "".to_string();
    for (s, t) in original {
        if max < t {
            max = t;
            result = s.clone();
        }
    }

    for (i, (s, _)) in sub.into_iter().enumerate() {
        if s == result {
            println!("{}", i + 1);
            return;
        }
    }
}
