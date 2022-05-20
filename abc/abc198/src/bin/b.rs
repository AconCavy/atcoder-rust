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
        mut n: i64,
    }

    if n == 0 {
        println!("Yes");
        return;
    }

    while n % 10 == 0 {
        n /= 10;
    }

    let mut s = Vec::new();
    while n > 0 {
        s.push(n % 10);
        n /= 10;
    }
    let mut result = true;
    for i in 0..s.len() {
        let j = s.len() - 1 - i;
        if i < j {
            result &= s[i] == s[j];
        } else {
            break;
        }
    }

    let answer = if result { "Yes" } else { "No" };
    println!("{}", answer);
}
