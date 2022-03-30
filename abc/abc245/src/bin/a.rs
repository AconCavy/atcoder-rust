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
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let mut answer = a < c;
    if a == c {
        answer = b <= d;
    }

    println!("{}", if answer { "Takahashi" } else { "Aoki" });
}
