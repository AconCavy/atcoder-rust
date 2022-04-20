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
        k: usize,
        c: [u64; n],
    }

    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(c[i]).or_insert(0) += 1;
    }
    let mut curr = map.len();
    let mut answer = curr;
    for i in k..n {
        let cc = c[i];
        let pc = c[i - k];
        *map.entry(cc).or_insert(0) += 1;
        if *map.get(&cc).unwrap() == 1 {
            curr += 1;
        }

        *map.entry(pc).or_insert(0) -= 1;
        if *map.get(&pc).unwrap() == 0 {
            curr -= 1;
        }
        answer = max(answer, curr);
    }

    println!("{}", answer);
}
