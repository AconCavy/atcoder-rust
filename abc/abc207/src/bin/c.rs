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
    }

    let mut lr = Vec::with_capacity(n);
    const DELTA: f64 = 0.25;
    for _ in 0..n {
        input! {
            t: usize,
            mut l: f64,
            mut r: f64,
        }

        match t {
            1 => {}
            2 => {
                r -= DELTA;
            }
            3 => {
                l += DELTA;
            }
            4 => {
                l += DELTA;
                r -= DELTA;
            }
            _ => {}
        }

        lr.push((l, r));
    }

    let mut answer = 0;
    for i in 0..n {
        for j in i + 1..n {
            let x = lr[i];
            let y = lr[j];
            if x.0 <= y.1 && y.0 <= x.1 {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}
