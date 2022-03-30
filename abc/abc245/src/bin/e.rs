#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use nalgebra::DimAdd;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;
use std::ops::Bound::{Included, Unbounded};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; m],
        d: [i32; m],
    }

    let mut elems = Vec::with_capacity(n + m);
    for i in 0..n {
        elems.push((0, a[i], b[i]));
    }

    for i in 0..m {
        elems.push((1, c[i], d[i]));
    }

    elems.sort_unstable_by(|l, r| r.1.cmp(&l.1).then(r.0.cmp(&l.0)));
    let mut set: BTreeSet<(i32, usize)> = BTreeSet::new();
    for (i, &e) in elems.iter().enumerate() {
        if e.0 == 0 {
            if let Some(&v) = set.range((Included((e.2, 0)), Unbounded)).next() {
                set.remove(&v);
            } else {
                println!("No");
                return;
            }
        } else {
            set.insert((e.2, i));
        }
    }

    println!("Yes");
}
