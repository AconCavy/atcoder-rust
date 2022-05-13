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
        a: [Chars; h],
    }

    let f = |i: usize, j: usize| if a[i][j] == '+' { 1 } else { -1 };
    let is_first = |i: usize, j: usize| (i + j) % 2 == 0;
    const INF: i32 = 1e9 as i32;
    let mut dp = vec![vec![INF; w]; h];
    for i in 0..h {
        for j in 0..w {
            dp[i][j] *= if is_first(i, j) { -1 } else { 1 };
        }
    }
    dp[h - 1][w - 1] = 0;

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if is_first(i, j) {
                if i + 1 < h {
                    dp[i][j] = max(dp[i][j], dp[i + 1][j] + f(i + 1, j));
                }
                if j + 1 < w {
                    dp[i][j] = max(dp[i][j], dp[i][j + 1] + f(i, j + 1));
                }
            } else {
                if i + 1 < h {
                    dp[i][j] = min(dp[i][j], dp[i + 1][j] - f(i + 1, j));
                }
                if j + 1 < w {
                    dp[i][j] = min(dp[i][j], dp[i][j + 1] - f(i, j + 1));
                }
            }
        }
    }

    let answer = match dp[0][0] {
        x if x > 0 => "Takahashi",
        x if x < 0 => "Aoki",
        _ => "Draw",
    };
    println!("{}", answer);
}
