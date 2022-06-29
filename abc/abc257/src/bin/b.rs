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
        k: usize,
        q: usize,
        a: [usize; k],
        l: [Usize1; q],
    }

    let mut b = a.into_iter().enumerate().map(|(i, a)| (a, i)).collect_vec();
    b.sort_by_key(|x| x.0);
    for l in l {
        if l + 1 < k {
            if b[l].0 + 1 != b[l + 1].0 {
                b[l].0 += 1;
            }
        } else {
            if b[l].0 < n {
                b[l].0 += 1;
            }
        }
    }

    b.sort_by_key(|x| x.1);
    println!("{}", b.into_iter().map(|x| x.0).join(" "));
}
