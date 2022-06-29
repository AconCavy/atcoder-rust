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
    }

    let mut jumps = Vec::new();
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
            p: i64,
        }

        jumps.push(Jump::new(x, y, p));
    }

    const INF: i64 = 4e9 as i64;
    let mut answer = INF;
    for i in 0..n {
        let f = |s: i64| -> bool {
            let mut queue = VecDeque::new();
            queue.push_back(i);
            let mut used = vec![false; n];
            used[i] = true;
            while let Some(u) = queue.pop_front() {
                for v in 0..n {
                    if !used[v] && jumps[u].can_jump(&jumps[v], s) {
                        used[v] = true;
                        queue.push_back(v);
                    }
                }
            }

            used.into_iter().all(|x| x)
        };

        let s = binary_search(0, INF, f);
        answer = min(answer, s);
    }

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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jump {
    x: i64,
    y: i64,
    p: i64,
}

impl Jump {
    pub fn new(x: i64, y: i64, p: i64) -> Self {
        Self { x, y, p }
    }

    pub fn can_jump(&self, to: &Jump, s: i64) -> bool {
        self.p * s >= (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}
