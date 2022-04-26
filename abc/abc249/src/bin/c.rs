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
        s: [Bytes; n],
    }

    let mut answer = 0;
    for i in 0..1 << n {
        let mut count = vec![0; 26];
        for j in 0..n {
            if (i >> j & 1) == 1 {
                for b in s[j].iter().map(|c| (c - b'a') as usize) {
                    count[b] += 1;
                }
            }
        }

        answer = max(answer, count.into_iter().filter(|&x| x == k).count());
    }

    println!("{}", answer);
}
