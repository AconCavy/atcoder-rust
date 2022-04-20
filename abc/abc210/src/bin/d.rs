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
        h: usize,
        w: usize,
        c: u64,
        mut a: [[u64; w]; h],
    }

    const INF: u64 = 2e18 as u64;
    let mut answer = INF;

    for _ in 0..2 {
        let mut dp = vec![vec![INF; w + 1]; h + 1];

        for i in (0..h).rev() {
            for j in (0..w).rev() {
                for &(di, dj) in &[(1, 0), (0, 1)] {
                    let (ni, nj) = (i + di, j + dj);
                    dp[i][j] = min(dp[i][j], dp[ni][nj] + c);
                }

                answer = min(answer, dp[i][j] + a[i][j]);
                dp[i][j] = min(dp[i][j], a[i][j]);
            }
        }

        for a in a.iter_mut() {
            a.reverse();
        }
    }

    println!("{}", answer);
}
