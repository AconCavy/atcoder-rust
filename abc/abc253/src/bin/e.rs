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
        m: usize,
        k: usize,
    }

    const MOD: i64 = 998_244_353;
    let mut dp0 = vec![0; m + 2];
    for j in 1..=m {
        dp0[j] = 1;
    }

    for _ in 0..n - 1 {
        let mut cum = vec![0; m + 2];
        for j in 1..=m {
            cum[j + 1] = (cum[j] + dp0[j]) % MOD;
        }

        let mut dp1 = vec![0; m + 2];
        for j in 1..=m {
            if k == 0 {
                dp1[j] = cum[m + 1];
            } else {
                let l0 = 0;
                let r0 = max(1, j as i32 - k as i32 + 1) as usize;
                let l1 = min(m + 1, j + k);
                let r1 = m + 1;
                dp1[j] = (cum[r0] - cum[l0] + cum[r1] - cum[l1]) % MOD;
                if dp1[j] < 0 {
                    dp1[j] += MOD;
                }
            }
        }
        dp0 = dp1;
    }

    let answer = dp0.into_iter().fold(0, |acc, x| (acc + x) % MOD);
    println!("{}", answer);
}
