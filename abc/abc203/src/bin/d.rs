#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use num_integer::Integer;
use num_traits::Signed;
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
        k: usize,
        a: [[i32; n]; n],
    }

    let mid = if k % 2 == 0 { k * k / 2 } else { k * k / 2 + 1 };

    let f = |x| -> bool {
        let mut cum = vec![vec![0; n + 1]; n + 1];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] <= x {
                    cum[i + 1][j + 1] += 1;
                }
            }
        }

        for i in 1..=n {
            for j in 1..=n {
                cum[i][j] += cum[i - 1][j] + cum[i][j - 1] - cum[i - 1][j - 1];
            }
        }

        for i in k..=n {
            for j in k..=n {
                if cum[i][j] - cum[i - k][j] - cum[i][j - k] + cum[i - k][j - k] >= mid {
                    return true;
                }
            }
        }
        false
    };
    const INF: i32 = 1e9 as i32;
    let answer = binary_search(-1, INF, f);
    println!("{}", answer);
}

pub fn binary_search<T: Copy + Integer + Signed, F: Fn(T) -> bool>(ng: T, ok: T, f: F) -> T {
    let mut ng = ng;
    let mut ok = ok;
    let two = T::one() + T::one();
    while (ok - ng).abs() > T::one() {
        let m = (ok + ng) / two;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}

pub fn lower_bound<T: Ord>(v: &[T], key: T) -> usize {
    let mut l = -1;
    let mut r = v.len() as isize;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if v[m as usize] >= key {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}
