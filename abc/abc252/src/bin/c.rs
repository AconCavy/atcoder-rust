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
        s: [Bytes; n],
    }

    const INF: i32 = 1e9 as i32;
    let mut idx = vec![vec![0; 10]; n];
    for i in 0..n {
        for j in 0..10 {
            idx[i][(s[i][j] - b'0') as usize] = j;
        }
    }

    let mut answer = INF;
    for x in 0..10 {
        let mut order = Vec::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                order.push(((10 * j + idx[i][x]) as i32, i));
            }
        }

        order.sort_by_key(|&t| t.0);
        let mut used = vec![false; n];
        let mut curr = -1;
        for (t, i) in order {
            if curr < t && !used[i] {
                used[i] = true;
                curr = t;
            }

            if used.iter().all(|&x| x) {
                answer = min(answer, t);
                break;
            }
        }
    }

    println!("{}", answer);
}
