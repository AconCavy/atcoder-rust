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
        mut a: [i32; n],
        mut b: [i32; m],
    }

    let am = a.into_iter().map(|x| (x, 'a'));
    let bm = b.into_iter().map(|x| (x, 'b'));

    let mut c: Vec<_> = am.merge(bm).collect();
    c.sort_unstable_by_key(|x| x.0);
    const INF: i32 = 1e9 as i32 + 10;
    let mut answer = INF;
    for (x, y) in c.into_iter().tuple_windows() {
        if x.1 != y.1 {
            answer = answer.min((x.0 - y.0).abs());
        }
    }

    println!("{}", answer);
}
