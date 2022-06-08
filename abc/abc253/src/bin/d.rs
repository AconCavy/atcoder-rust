#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use num_integer::lcm;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }

    let f = |x| x * (x + 1) / 2;
    let lcm = lcm(a, b);
    let answer = f(n) - a * f(n / a) - b * f(n / b) + lcm * f(n / lcm);
    println!("{}", answer);
}
