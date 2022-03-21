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
        s: [char; 3],
        t: [char; 3],
    }

    let mut x = 0;
    for (s, t) in s.iter().zip(t.iter()) {
        if s != t {
            x += 1;
        }
    }

    let answer = x != 2;
    println!("{}", if answer { "Yes" } else { "No" });
}
