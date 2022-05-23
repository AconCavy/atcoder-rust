#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::iter::FromIterator;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    }

    let mut heap = BinaryHeap::from_iter(a.iter().map(|&x| Reverse(x)));
    let sum = a.into_iter().fold(0, |acc, x| acc + x);
    if sum < l {
        heap.push(Reverse(l - sum));
    }

    let mut answer = 0;
    while heap.len() > 1 {
        let Reverse(l0) = heap.pop().unwrap();
        let Reverse(l1) = heap.pop().unwrap();
        answer += l0 + l1;
        heap.push(Reverse(l0 + l1));
    }

    println!("{}", answer);
}
