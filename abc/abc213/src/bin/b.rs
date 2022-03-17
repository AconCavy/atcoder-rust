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
        a: [i32; n],
    }

    let mut b: Vec<_> = a.iter().enumerate().collect();
    b.sort_unstable_by_key(|x| x.1);
    b.reverse();
    let answer = b[1].0 + 1;
    println!("{}", answer);
}
