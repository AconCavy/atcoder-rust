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
        a: [Chars; n],
    }

    let mut answer = true;
    for i in 0..n {
        for j in i + 1..n {
            answer &= match (a[i][j], a[j][i]) {
                ('W', 'L') => true,
                ('L', 'W') => true,
                ('D', 'D') => true,
                _ => false,
            };
        }
    }

    println!("{}", if answer { "correct" } else { "incorrect" });
}
