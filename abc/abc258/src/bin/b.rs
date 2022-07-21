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
        a: [Bytes; n],
    }

    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[i][j] - b'0';
        }
    }

    let d8: [(i64, i64); 8] = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let nn = n as i64;
    let mut answer = 0;
    for i in 0..nn {
        for j in 0..nn {
            for &(di, dj) in &d8 {
                let mut sum = 0;
                for k in 0..nn {
                    let ii = ((i + nn + di * k) % nn) as usize;
                    let jj = ((j + nn + dj * k) % nn) as usize;
                    sum = sum * 10 + b[ii][jj] as i64;
                }
                answer = max(answer, sum);
            }
        }
    }

    println!("{}", answer);
}
