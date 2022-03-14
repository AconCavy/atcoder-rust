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
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    }

    v %= a + b + c;
    if v < a {
        println!("F");
    } else if v < a + b {
        println!("M");
    } else {
        println!("T");
    }
}
