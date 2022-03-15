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
        m: usize,
        a: [i32; n],
    }

    let mut p = a.iter().flat_map(|x| get_factors(*x)).collect_vec();
    p.sort();
    p.dedup();
    let mut ok = vec![true; m + 1];
    for p in p {
        let mut i = p;
        loop {
            if i as usize > m {
                break;
            }

            ok[i as usize] = false;
            i += p;
        }
    }

    let answer = ok
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x)
        .map(|(i, _)| i)
        .skip(1)
        .collect_vec();
    println!("{}", answer.len());
    println!("{}", answer.iter().join("\n"));
}

fn get_factors(x: i32) -> Vec<i32> {
    let mut result = Vec::new();
    if x < 2 {
        return result;
    }

    let mut x = x;
    if x % 2 == 0 {
        result.push(2);
    }
    while x % 2 == 0 {
        x /= 2;
    }

    let mut i = 3;
    loop {
        if i * i > x {
            break;
        }

        if x % i == 0 {
            result.push(i as i32);
        }

        while x % i == 0 {
            x /= i;
        }

        i += 2;
    }

    if x > 1 {
        result.push(x as i32);
    }

    return result;
}
