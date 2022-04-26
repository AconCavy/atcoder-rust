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
    }

    let mut ok = s.iter().any(|c| c.is_uppercase()) && s.iter().any(|c| c.is_lowercase());
    let n = s.len();
    s.sort();
    s.dedup();
    ok &= s.len() == n;
    let answer = if ok { "Yes" } else { "No" };
    println!("{}", answer);
}
