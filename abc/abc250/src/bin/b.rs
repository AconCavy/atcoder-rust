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
        a: usize,
        b: usize,
    }

    for i in 0..n * a {
        for j in 0..n * b {
            let x = match (i / a + j / b) % 2 {
                0 => '.',
                1 => '#',
                _ => '.',
            };

            print!("{}", x);
        }

        println!();
    }
}
