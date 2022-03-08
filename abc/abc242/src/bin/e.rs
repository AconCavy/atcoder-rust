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
        q: usize,
    }

    const M: i64 = 998244353;

    for _ in 0..q {
        input! {
            n: usize,
            s: Bytes,
        }

        let half = (n + 1) / 2;
        let mut answer = 0i64;
        for i in 0..half {
            answer *= 26;
            answer += (s[i] - b'A') as i64;
            answer %= M;
        }

        let mut t = s.clone();
        for i in 0..half {
            t[n - 1 - i] = t[i];
        }

        if t <= s {
            answer += 1;
        }

        answer %= M;
        println!("{}", answer);
    }
}
