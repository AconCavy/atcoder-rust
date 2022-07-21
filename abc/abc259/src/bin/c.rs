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
        s: Chars,
        t: Chars,
    }

    fn f(s: &[char]) -> Vec<(char, usize)> {
        let mut result = Vec::new();
        let mut l = 0;
        while l < s.len() {
            let mut r = l;
            while r < s.len() && s[r] == s[l] {
                r += 1;
            }
            result.push((s[l], r - l));
            l = r;
        }

        result
    }

    let fs = f(&s);
    let ft = f(&t);

    let mut answer = fs.len() == ft.len();
    if answer {
        for i in 0..fs.len() {
            let (sv, sc) = fs[i];
            let (tv, tc) = ft[i];
            answer &= sv == tv && (sc == 1 && tc == 1 || sc >= 2 && sc <= tc);
        }
    }

    println!("{}", if answer { "Yes" } else { "No" });
}
