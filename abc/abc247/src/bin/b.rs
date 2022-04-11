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

    let mut s = Vec::with_capacity(n);
    let mut t = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            si: String,
            ti: String,
        }

        s.push(si);
        t.push(ti);
    }

    let mut ok = true;
    for i in 0..n {
        let mut ok_s = true;
        let mut ok_t = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            ok_s &= s[i] != s[j] && s[i] != t[j];
            ok_t &= t[i] != s[j] && t[i] != t[j];
        }

        ok &= ok_s || ok_t;
    }

    let answer = if ok { "Yes" } else { "No" };
    println!("{}", answer);
}
