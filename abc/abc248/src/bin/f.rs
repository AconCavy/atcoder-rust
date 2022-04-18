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
        p: u64,
    }

    // dp[i][j][k] := 連結しているグラフのうちi列目までにj本の辺を削除したときの上下状態(k=0:非連結, k=1: 連結)
    let mut dp = vec![vec![vec![0; 2]; n + 2]; n + 1];
    dp[1][0][1] = 1;
    dp[1][1][0] = 1;
    for i in 2..=n {
        for j in 0..n {
            // Remove: None
            dp[i][j][1] += dp[i - 1][j][0];
            dp[i][j][1] += dp[i - 1][j][1];
            dp[i][j][0] %= p;
            dp[i][j][1] %= p;

            // Remove: U
            dp[i][j + 1][1] += dp[i - 1][j][1];
            // Remove: B
            dp[i][j + 1][1] += dp[i - 1][j][1];
            // Remove: R
            dp[i][j + 1][0] += dp[i - 1][j][0];
            dp[i][j + 1][1] += dp[i - 1][j][1];

            // Remove: UR
            dp[i][j + 2][0] += dp[i - 1][j][1];
            // Remove: BR
            dp[i][j + 2][0] += dp[i - 1][j][1];

            // Remove: UB

            // Remove: URB
        }
    }

    let answer = dp[n].iter().skip(1).take(n - 1).map(|v| v[1]).join(" ");
    println!("{}", answer);
}
