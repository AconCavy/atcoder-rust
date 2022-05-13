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
    }

    let mut answer = 0;
    for b0 in 0..10 {
        for b1 in 0..10 {
            for b2 in 0..10 {
                for b3 in 0..10 {
                    let mut ok = true;
                    let mut buffer = vec!['x'; 10];
                    buffer[b0] = 'o';
                    buffer[b1] = 'o';
                    buffer[b2] = 'o';
                    buffer[b3] = 'o';
                    for i in 0..10 {
                        if s[i] == 'o' {
                            ok &= buffer[i] == 'o';
                        } else if s[i] == 'x' {
                            ok &= buffer[i] == 'x';
                        } else {
                            ok &= buffer[i] == 'o' || buffer[i] == 'x';
                        }
                    }

                    if ok {
                        answer += 1;
                    }
                }
            }
        }
    }

    println!("{}", answer);
}
