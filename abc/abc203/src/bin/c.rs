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
        k: i64,
        mut friends: [(i64, i64); n],
    }

    friends.sort();
    let mut answer = k;
    for (a, b) in friends {
        if answer < a {
            break;
        }
        answer += b;
    }

    println!("{}", answer);
}
