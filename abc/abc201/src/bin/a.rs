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
        mut a: [i32; 3]
    }

    a.sort();
    let answer = if a[1] - a[0] == a[2] - a[1] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", answer);
}
