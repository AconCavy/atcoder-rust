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

    const MOD: u64 = 998244353;
    let mut dp1 = vec![0; k + 1];
    dp1[0] = 1;
    for _ in 0..n {
        let mut dp2 = vec![0; k + 1];
        for kk in 0..=k {
            for mm in (1..=m).take_while(|mm| kk + mm <= k) {
                dp2[kk + mm] += dp1[kk];
                dp2[kk + mm] %= MOD;
            }
        }

        dp1 = dp2;
    }

    let answer = dp1
        .into_iter()
        .skip(1)
        .take(k)
        .fold(0, |acc, x| (acc + x) % MOD);
    println!("{}", answer);
}
