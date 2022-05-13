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
        mut a: usize,
        mut b: usize,
        mut k: i64,
    }

    let n = a + b;
    let mut comb = vec![vec![0i64; n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=i {
            comb[i][j] = if j == 0 || j == i {
                1
            } else {
                comb[i - 1][j - 1] + comb[i - 1][j]
            }
        }
    }

    let mut answer = vec!['-'; n];
    for ans in answer.iter_mut() {
        if a > 0 {
            if k <= comb[a + b - 1][b] {
                *ans = 'a';
                a -= 1;
            } else {
                *ans = 'b';
                k -= comb[a + b - 1][b];
                b -= 1;
            }
        } else {
            *ans = 'b';
            b -= 1;
        }
    }
    println!("{}", answer.iter().join(""));
}
