#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::mem::*;
use std::{char, io};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let answer = char::from(b'A' + ((x - 1) / n) as u8);
    println!("{}", answer);
}
