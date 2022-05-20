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
        s1: Bytes,
        s2: Bytes,
        s3: Bytes,
    }

    let s1 = s1
        .into_iter()
        .map(|c| (c - b'a') as usize)
        .collect::<Vec<_>>();
    let s2 = s2
        .into_iter()
        .map(|c| (c - b'a') as usize)
        .collect::<Vec<_>>();
    let s3 = s3
        .into_iter()
        .map(|c| (c - b'a') as usize)
        .collect::<Vec<_>>();

    let mut set = HashSet::new();
    for &c in &s1 {
        set.insert(c);
    }
    for &c in &s2 {
        set.insert(c);
    }
    for &c in &s3 {
        set.insert(c);
    }

    if set.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let list = set.into_iter().collect::<Vec<_>>();
    const INF: i64 = 1e18 as i64;

    for perm in (0..10).permutations(list.len()) {
        let mut map = vec![INF; 26];
        for (i, &c) in list.iter().enumerate() {
            map[c] = perm[i];
        }

        let f = |s: &[usize]| {
            s.iter().fold(
                0,
                |acc, &c| if acc == INF { INF } else { acc * 10 + map[c] },
            )
        };

        let mut ok = true;
        ok &= map[s1[0]] != 0;
        ok &= map[s2[0]] != 0;
        ok &= map[s3[0]] != 0;
        let v1 = f(&s1);
        let v2 = f(&s2);
        let v3 = f(&s3);
        ok &= v1 + v2 == v3;
        if ok {
            println!("{}\n{}\n{}", v1, v2, v3);
            return;
        }
    }

    println!("UNSOLVABLE");
}
