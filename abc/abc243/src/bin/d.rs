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
        x: i64,
        s: Chars,
    }

    let mut buffer = Vec::with_capacity(n);
    for &c in &s {
        if c == 'U' && buffer.last().map_or(false, |&x| x != 'U') {
            buffer.pop();
        } else {
            buffer.push(c);
        }
    }

    let mut answer = x;
    for &c in &buffer {
        if c == 'U' {
            answer /= 2;
        } else if c == 'L' {
            answer *= 2;
        } else {
            answer *= 2;
            answer += 1;
        }
    }

    println!("{}", answer);
}
