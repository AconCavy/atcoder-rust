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
        _: usize,
        _: usize,
        n: usize,
        p: [(i32, i32); n],
    }

    let vec_h: Vec<_> = p.iter().map(|x| x.1).collect();
    let vec_w: Vec<_> = p.iter().map(|x| x.0).collect();
    let (map_h, _) = compress(&vec_h);
    let (map_w, _) = compress(&vec_w);
    for (w, h) in p {
        let cw = map_w.get(&w).unwrap() + 1;
        let ch = map_h.get(&h).unwrap() + 1;
        println!("{} {}", cw, ch);
    }
}

fn compress<T: std::cmp::Ord + std::hash::Hash + std::cmp::Eq + Clone>(
    source: &[T],
) -> (HashMap<T, usize>, HashMap<usize, T>) {
    let set: HashSet<_> = source.iter().collect();
    let mut map: HashMap<T, usize> = HashMap::new();
    let mut remap: HashMap<usize, T> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        map.insert(x.clone(), i);
        remap.insert(i, x.clone());
    }

    (map, remap)
}
