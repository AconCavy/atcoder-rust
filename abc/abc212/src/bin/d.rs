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

    let mut heap = BinaryHeap::new();
    let mut cum = 0i64;

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: i64,
            }
            heap.push(Reverse(x - cum));
        } else if t == 2 {
            input! {
                x: i64,
            }
            cum += x;
        } else {
            let answer = heap.pop().map_or(0, |Reverse(x)| x + cum);
            println!("{}", answer);
        }
    }
}
