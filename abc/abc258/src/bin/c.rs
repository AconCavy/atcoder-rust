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
        q: usize,
        s: Chars,
    }

    let mut idx = 0;
    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: usize,
            }

            idx = (idx + n - x % n) % n;
        } else {
            input! {
                x: Usize1,
            }

            println!("{}", s[(idx + x) % n]);
        }
    }
}
