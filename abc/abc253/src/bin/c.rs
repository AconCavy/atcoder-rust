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
        q: usize,
    }

    let mut set = BTreeSet::new();
    let mut map = HashMap::new();
    let mut answer = Vec::new();

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            1 => {
                input! {
                    x: i64,
                }

                set.insert(x);
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: i64,
                    c: i64,
                }

                let count = map.entry(x).or_insert(0);
                *count -= min(*count, c);
                if *count == 0 {
                    set.remove(&x);
                }
            }
            3 => {
                let max = set.iter().next_back().unwrap();
                let min = set.iter().next().unwrap();
                answer.push(max - min);
            }
            _ => {}
        }
    }

    println!("{}", answer.iter().join("\n"));
}
