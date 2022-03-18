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
    }

    if a > 0 && b == 0 {
        println!("Gold");
    } else if a == 0 && b > 0 {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
