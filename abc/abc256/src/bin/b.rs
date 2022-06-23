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
        a: [usize; n],
    }

    let mut count = vec![0; 5];
    for a in a {
        count[0] += 1;
        for i in (0..4).rev() {
            count[min(4, i + a)] += count[i];
            count[i] = 0;
        }
    }

    let answer = count[4];
    println!("{}", answer);
}
