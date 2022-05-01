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
        s: [String; n],
    }

    let mut map = HashMap::new();
    let mut e = Vec::with_capacity(n);
    let mut m = 0;
    for s in &s {
        let head = s[..3].to_string();
        let tail = s[s.len() - 3..].to_string();
        map.entry(head.clone()).or_insert_with(|| {
            m += 1;
            m - 1
        });

        map.entry(tail.clone()).or_insert_with(|| {
            m += 1;
            m - 1
        });

        e.push((head, tail));
    }

    let mut g = vec![Vec::new(); m];
    let mut deg = vec![0; m];
    for (head, tail) in e.iter() {
        let u = map.get(head).unwrap();
        let v = map.get(tail).unwrap();
        g[*v].push(*u);
        deg[*u] += 1;
    }

    let mut dp = vec![-1; m];
    let mut queue = VecDeque::new();
    for u in 0..m {
        if deg[u] == 0 {
            dp[u] = 0;
            queue.push_back(u);
        }
    }

    while let Some(u) = queue.pop_front() {
        for &v in &g[u] {
            if dp[v] == -1 {
                deg[v] -= 1;
                if dp[u] == 0 {
                    dp[v] = 1;
                    queue.push_back(v);
                } else if deg[v] == 0 {
                    dp[v] = 0;
                    queue.push_back(v);
                }
            }
        }
    }

    for (_, tail) in e {
        let u = map.get(&tail).unwrap();
        let answer = match dp[*u] {
            0 => "Takahashi",
            1 => "Aoki",
            _ => "Draw",
        };

        println!("{}", answer);
    }
}
