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
        s: Bytes,
    }

    const MOD: i32 = 998244353;
    let m = 10;
    let s: Vec<_> = s.iter().map(|x| (x - b'A') as usize).collect();
    let mut dp = vec![vec![vec![0; m]; 1 << m]; n + 1];
    for i in 0..n {
        dp[i + 1][1 << s[i]][s[i]] += 1;
        for j in 0..(1 << m) {
            for k in 0..m {
                dp[i + 1][j][k] += dp[i][j][k];
                dp[i + 1][j][k] %= MOD;

                if k == s[i] {
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= MOD;
                }
            }
        }

        for j in 0..(1 << m) {
            if (j >> s[i] & 1) == 1 {
                continue;
            }

            for k in 0..m {
                dp[i + 1][j | (1 << s[i])][s[i]] += dp[i][j][k];
                dp[i + 1][j | (1 << s[i])][s[i]] %= MOD;
            }
        }
    }

    let mut answer = 0;
    for j in 1..(1 << m) {
        for k in 0..m {
            answer += dp[n][j][k];
            answer %= MOD;
        }
    }

    println!("{}", answer);
}
