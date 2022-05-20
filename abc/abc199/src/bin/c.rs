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
        mut s: Chars,
        q: usize,
    }

    let mut flip = false;
    for _ in 0..q {
        input! {
            t: usize,
            mut a: usize,
            mut b: usize,
        }

        if t == 1 {
            if flip {
                if a <= n {
                    a += n;
                } else {
                    a -= n;
                }
                if b <= n {
                    b += n;
                } else {
                    b -= n;
                }
            }
            s.swap(a - 1, b - 1);
        } else {
            flip = !flip;
        }
    }

    if flip {
        for i in 0..n {
            s.swap(i, i + n);
        }
    }

    let answer = s.into_iter().join("");
    println!("{}", answer);
}
