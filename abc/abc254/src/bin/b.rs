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
    }

    let coef = bin_coef(n);
    for i in 0..n {
        for j in 0..=i {
            print!("{}", coef[i][j]);
            print!("{}", if j == i { '\n' } else { ' ' });
        }
    }
}

pub fn bin_coef(n: usize) -> Vec<Vec<i64>> {
    let mut result = vec![vec![0i64; n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=i {
            result[i][j] = if j == 0 || j == i {
                1
            } else {
                result[i - 1][j - 1] + result[i - 1][j]
            }
        }
    }

    result
}
