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
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ok = false;
    let mut ii = h;
    let mut jj = w;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                if ok {
                    let answer = (ii as i32 - i as i32).abs() + (jj as i32 - j as i32).abs();
                    println!("{}", answer);
                    return;
                } else {
                    ii = i;
                    jj = j;
                    ok = true;
                }
            }
        }
    }
}
