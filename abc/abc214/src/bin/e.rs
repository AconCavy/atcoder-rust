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
        t: usize,
    }

    const INF: i32 = 1e9 as i32 + 10;

    for _ in 0..t {
        input! {
            n: usize,
            mut range: [(i32, i32); n],
        }

        range.push((INF, INF));
        range.sort_unstable();

        let mut heap = BinaryHeap::new();
        let mut curr = 1;
        let mut answer = true;
        for (l, r) in range {
            while answer && curr < l && !heap.is_empty() {
                if heap.pop().map_or(false, |Reverse(p)| p < curr) {
                    answer = false;
                    break;
                }
                curr += 1;
            }

            curr = l;
            heap.push(Reverse(r));
        }

        println!("{}", if answer { "Yes" } else { "No" });
    }
}
