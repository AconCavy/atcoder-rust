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
        mut k: i64,
        mut a: [i64; n],
    }

    let mut heap = BinaryHeap::new();
    const INF: i64 = 1e18 as i64;
    heap.push((0, INF));
    for a in a {
        heap.push((a, 1));
    }

    let f = |x| x * (x + 1) / 2;

    let mut answer = 0;
    while let Some((v1, c1)) = heap.pop() {
        if let Some((v2, c2)) = heap.pop() {
            let c = c1 * (v1 - v2);
            if c <= k {
                answer += (f(v1) - f(v2)) * c1;
                heap.push((v2, c2 + c1));
                k -= c;
            } else {
                let d = k / c1;
                answer += (f(v1) - f(v1 - d)) * c1;
                answer += (v1 - d) * (k % c1);
                break;
            }
        }
    }

    println!("{}", answer);
}
