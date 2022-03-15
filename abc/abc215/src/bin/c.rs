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
        mut s: Chars,
        k: Usize1,
    }

    let mut t = s.iter().permutations(s.len()).collect_vec();
    t.sort();
    t.dedup();
    let answer = t[k].iter().join("");
    println!("{}", answer);
}
