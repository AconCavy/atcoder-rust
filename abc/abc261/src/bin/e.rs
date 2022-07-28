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
        c: i32,
        ops: [(usize, i32); n],
    }

    let mut answer = vec![0; n];
    for i in 0..30 {
        let mut bit_c = c >> i & 1;
        let mut cum = [0, 1];
        for j in 0..n {
            let (t, x) = ops[j];
            let mut f = [0, 1];
            let bit_x = x >> i & 1;
            if t == 1 {
                f[0] &= bit_x;
                f[1] &= bit_x;
            }
            if t == 2 {
                f[0] |= bit_x;
                f[1] |= bit_x;
            }
            if t == 3 {
                f[0] ^= bit_x;
                f[1] ^= bit_x;
            }

            cum[0] = f[cum[0] as usize];
            cum[1] = f[cum[1] as usize];
            bit_c = cum[bit_c as usize];
            answer[j] |= bit_c << i;
        }
    }

    println!("{}", answer.into_iter().join("\n"));
}
