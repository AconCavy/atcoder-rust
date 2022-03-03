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
        xy:Chars,
    }

    let len = xy.len();
    let x = &xy[..(len - 2)].iter().join("");
    let y = match xy[len - 1].to_digit(10).unwrap() {
        0...2 => "-",
        3...6 => "",
        7...9 => "+",
        _ => "",
    };

    println!("{}{}", x, y);
}
