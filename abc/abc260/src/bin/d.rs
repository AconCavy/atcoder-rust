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
        k: usize,
        p: [usize; n],
    }

    let mut answer = vec![-1; n + 1];
    let mut set = BTreeSet::new();
    let mut map = HashMap::new();
    let mut root = vec![0; n + 1];
    for i in 0..n {
        let p = p[i];
        root[p] = p;
        let y = *set.range((Included(p), Unbounded)).next().unwrap_or(&p);
        root[p] = root[y];
        let r = root[p];
        set.remove(&y);
        set.insert(p);
        map.entry(r).or_insert_with(Vec::new).push(p);
        if map.get(&r).unwrap().len() >= k {
            for &v in map.get(&r).unwrap() {
                answer[v] = i as i32 + 1;
            }
            set.remove(&p);
            map.remove(&r);
        }
    }

    println!("{}", answer.into_iter().skip(1).join("\n"));
}
