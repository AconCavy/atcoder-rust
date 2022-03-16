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
        s: i64,
        t: i64,
    }

    let mut answer = 0;
    let mut a = 0;
    while a <= s {
        let mut b = 0;
        while a + b <= s {
            let x = s - (a + b);
            let y = if a * b == 0 { x } else { t / (a * b) };
            answer += x.min(y) + 1;
            b += 1;
        }
        a += 1;
    }

    println!("{}", answer);
}
