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

    let mut deq = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                x: u64,
                c: u64,
            }
            deq.push_back((x, c));
        } else {
            input! {
                mut c: u64,
            }

            let mut sum = 0;
            while c > 0 {
                let (xx, cc) = deq.pop_front().unwrap();
                let cnt = cc.min(c);
                sum += xx * cnt;
                c -= cnt;
                if c == 0 {
                    deq.push_front((xx, cc - cnt));
                }
            }

            println!("{}", sum);
        }
    }
}
