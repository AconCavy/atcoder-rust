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
        mut n: usize,
        q: usize,
        mut a: [i64; n],
    }

    for _ in 0..q {
        input! {
            k: i64,
        }

        let answer = if k < a[0] {
            k
        } else if k >= a[n - 1] {
            k + n as i64
        } else {
            let mut curr = k;
            let mut prev = -1;
            while curr != prev {
                let ub = upper_bound(&a, curr);
                prev = curr;
                curr = k + ub as i64;
            }
            curr
        };

        println!("{}", answer);
    }
}

pub fn upper_bound<T: Ord>(v: &[T], key: T) -> usize {
    let mut l = -1;
    let mut r = v.len() as isize;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if v[m as usize] > key {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}
