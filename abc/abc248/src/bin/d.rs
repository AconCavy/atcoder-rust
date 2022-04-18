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
        a: [u64; n],
        q: usize,
    }

    let mut map = HashMap::new();
    for (i, a) in a.into_iter().enumerate() {
        map.entry(a).or_insert(Vec::new()).push(i);
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            x: u64,
        }

        let answer = map.get(&x).map_or(0, |v| {
            let l = lower_bound(v, l);
            let r = upper_bound(v, r);
            r - l
        });
        println!("{}", answer);
    }
}

fn lower_bound(v: &[usize], key: usize) -> usize {
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

fn upper_bound(v: &[usize], key: usize) -> usize {
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
