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
    }

    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            s: String,
        }

        let count = map.entry(s.clone()).or_insert(0);
        if *count == 0 {
            println!("{}", s);
        } else {
            println!("{}({})", s, *count);
        }

        *count += 1;
    }
}
