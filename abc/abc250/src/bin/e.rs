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

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        q: usize,
    }

    let mut map_a = HashMap::new();
    let mut map_b = HashMap::new();
    let inf = n + 10;

    for i in (0..n).rev() {
        *map_a.entry(b[i]).or_insert(inf) = i + 1;
        *map_b.entry(a[i]).or_insert(inf) = i + 1;
    }

    let mut cum_a = vec![0; n + 1];
    let mut cum_b = vec![0; n + 1];
    for i in 0..n {
        cum_a[i + 1] = max(cum_a[i], *map_a.entry(a[i]).or_insert(inf));
        cum_b[i + 1] = max(cum_b[i], *map_b.entry(b[i]).or_insert(inf));
    }

    for _ in 0..q {
        input! {
            x: usize,
            y: usize,
        }

        let answer = if cum_a[x] <= y && cum_b[y] <= x {
            "Yes"
        } else {
            "No"
        };
        println!("{}", answer);
    }
}
