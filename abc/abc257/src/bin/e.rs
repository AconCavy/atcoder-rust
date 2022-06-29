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
        mut n: i64,
    }

    const INF: i64 = 1e18 as i64;
    let mut c = vec![0; 10];
    c[0] = INF;
    for i in 1..10 {
        input! {
            v: i64,
        }
        c[i] = v;
    }

    let min = *c.iter().min().unwrap();
    let len = n / min;
    let mut answer = vec![0; len as usize];
    for i in 0..len {
        for j in (1..10).rev() {
            if min * (len - 1 - i) + c[j] <= n {
                n -= c[j];
                answer[i as usize] = j;
            }
        }
    }

    println!("{}", answer.into_iter().join(""));
}
