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
    }

    let mut is_square = vec![false; n + 1];
    for i in (1..).take_while(|i| i * i <= n) {
        is_square[i * i] = true;
    }

    let mut count = vec![0; n + 1];
    for i in 1..=n {
        let max = get_divisors(i)
            .into_iter()
            .filter(|&d| is_square[d])
            .fold(0, max);
        count[i / max] += 1;
    }

    let answer = count.into_iter().fold(0, |acc, x| acc + x * x);
    println!("{}", answer);
}

fn get_divisors(n: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for i in (1..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            result.push(i);
            if i != n / i {
                result.push(n / i);
            }
        }
    }

    result
}
