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
        m: usize,
        mut e: [(i64, i64); m],
    }

    e.sort_by_key(|&x| x.1);
    let mut curr = n as i64;
    let mut answer = 0;
    for (a, c) in e {
        let g = gcd(curr, a);
        answer += (curr - g) * c;
        curr = g;
    }

    if curr == 1 {
        println!("{}", answer);
    } else {
        println!("-1");
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    return if b == 0 { a } else { gcd(b, a % b) };
}
