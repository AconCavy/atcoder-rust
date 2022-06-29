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
        s: Chars,
        w: [i64; n],
    }

    let mut w0 = Vec::new();
    let mut w1 = Vec::new();
    for i in 0..n {
        if s[i] == '0' {
            w0.push(w[i]);
        } else {
            w1.push(w[i]);
        }
    }

    w0.sort();
    w1.sort();

    let mut answer = max(w0.len(), w1.len());
    for w in w {
        let c0 = lower_bound(&w0, w);
        let c1 = w1.len() - lower_bound(&w1, w);
        answer = max(answer, c0 + c1);
    }

    println!("{}", answer);
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
