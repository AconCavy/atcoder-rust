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
        mut x: usize,
        mut y: usize,
        mut z: usize,
        a: [i32; n],
        b: [i32; n],
    }

    let mut c = a
        .into_iter()
        .zip(b.into_iter())
        .enumerate()
        .map(|(i, (a, b))| (i, a, b))
        .collect_vec();

    let mut accepts = vec![false; n];
    c.sort_by_key(|x| x.1 * 10000 - x.0 as i32);
    c.reverse();
    for &(i, _, _) in &c {
        if x > 0 && !accepts[i] {
            accepts[i] = true;
            x -= 1;
        }
    }

    c.sort_by_key(|x| x.2 * 10000 - x.0 as i32);
    c.reverse();
    for &(i, _, _) in &c {
        if y > 0 && !accepts[i] {
            accepts[i] = true;
            y -= 1;
        }
    }

    c.sort_by_key(|x| (x.1 + x.2) * 10000 - x.0 as i32);
    c.reverse();
    for &(i, _, _) in &c {
        if z > 0 && !accepts[i] {
            accepts[i] = true;
            z -= 1;
        }
    }

    for i in 0..n {
        if accepts[i] {
            println!("{}", i + 1);
        }
    }
}
