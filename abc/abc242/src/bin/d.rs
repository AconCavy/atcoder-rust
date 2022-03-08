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
        q: usize,
    }

    fn encode(b: u8) -> char {
        std::char::from_u32(b as u32).unwrap()
    }

    fn g(c: char, x: u64) -> char {
        let c = (c as u8 - b'A') as u64;
        let b = ((c + x) % 3) as u8;
        encode(b + b'A')
    }

    fn f(s: &Vec<char>, t: u64, k: u64) -> char {
        return if t == 0 {
            s[k as usize]
        } else if k == 0 {
            g(s[0], t)
        } else {
            g(f(s, t - 1, k / 2), k % 2 + 1)
        };
    }

    for _ in 0..q {
        input! {
            t: u64,
            k: u64,
        }

        let answer = f(&s, t, k - 1);
        println!("{}", answer);
    }
}
