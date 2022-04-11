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
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let f = |a: &[usize]| -> usize {
        let mut used = vec![0; x + 1];
        let mut result = 0;
        let mut l = 0;
        for r in 0..a.len() {
            used[a[r]] += 1;
            while l <= r && used[x] > 0 && used[y] > 0 {
                result += a.len() - r;
                used[a[l]] -= 1;
                l += 1;
            }
        }

        result
    };

    let mut answer = 0;
    let mut buffer = Vec::new();
    for a in a {
        if y <= a && a <= x {
            buffer.push(a);
        } else {
            answer += f(&buffer);
            buffer.clear();
        }
    }

    answer += f(&buffer);
    println!("{}", answer);
}
