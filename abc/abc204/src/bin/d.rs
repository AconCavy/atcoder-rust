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
        t: [usize; n],
    }
    let sum = t.iter().sum::<usize>();
    let mut dp = vec![false; sum + 1];
    dp[0] = true;
    for t in t {
        for i in (0..=sum - t).rev() {
            dp[i + t] |= dp[i];
        }
    }

    let mut answer = 1000 * n;
    for i in 0..=sum / 2 {
        let j = sum - i;
        if dp[i] && dp[j] {
            answer = min(answer, max(i, j));
        }
    }

    println!("{}", answer);
}
