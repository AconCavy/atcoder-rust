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
    }

    let mut queues = vec![VecDeque::new(); m];
    let mut colors = vec![VecDeque::new(); n];
    for i in 0..m {
        input! {
            k: usize,
            a: [Usize1; k],
        }

        colors[a[0]].push_back(i);
        for a in a {
            queues[i].push_back(a);
        }
    }

    let mut pairs = VecDeque::new();
    for color in colors.iter_mut() {
        if color.len() == 2 {
            pairs.push_back((color.pop_front().unwrap(), color.pop_front().unwrap()));
        }
    }

    while let Some((a, b)) = pairs.pop_front() {
        queues[a].pop_front();
        queues[b].pop_front();

        for &i in &[a, b] {
            if let Some(&idx) = queues[i].front() {
                colors[idx].push_back(i);
                if colors[idx].len() == 2 {
                    pairs.push_back((
                        colors[idx].pop_front().unwrap(),
                        colors[idx].pop_front().unwrap(),
                    ));
                }
            }
        }
    }

    let answer = queues.iter().all(|x| x.is_empty());
    println!("{}", if answer { "Yes" } else { "No" });
}
