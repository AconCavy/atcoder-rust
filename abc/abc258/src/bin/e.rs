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
        q: usize,
        x: i64,
        w: [i64; n],
    }

    let sum = w.iter().fold(0, |acc, &v| acc + v);
    let xx = x % sum;

    let mut count = vec![0; n];
    let mut next = vec![0; n];
    {
        let mut cum = 0;
        let mut r = 0;
        for l in 0..n {
            while cum < xx {
                cum += w[r % n];
                r += 1;
            }

            count[l] = x / sum * n as i64 + (r - l) as i64;
            cum -= w[l];
            next[l] = (l + (count[l] % n as i64) as usize) % n;
        }
    }

    let mut steps = Vec::new();
    let mut loop_length = 0;
    let mut no_loop_length = 0;
    let mut map = HashMap::new();
    let mut curr = 0;
    for i in 0.. {
        if map.contains_key(&curr) {
            no_loop_length = *map.entry(curr).or_default();
            loop_length = steps.len() - no_loop_length;
            break;
        }

        map.insert(curr, i);
        steps.push(curr);
        curr = next[curr];
    }

    for _ in 0..q {
        input! {
            k: Usize1,
        }

        let answer = if k <= no_loop_length {
            count[steps[k]]
        } else {
            count[steps[no_loop_length + (k - no_loop_length) % loop_length]]
        };
        println!("{}", answer);
    }
}
