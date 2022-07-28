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
        s: Bytes,
    }

    let mut count = vec![0; 26];
    for c in s {
        count[(c - b'a') as usize] += 1;
    }

    for i in 0..26 {
        if count[i] == 1 {
            println!("{}", (b'a' + i as u8) as char);
            return;
        }
    }

    println!("-1");
}
