#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::io;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        r: Usize1,
        c: Usize1,
    }

    let mut answer = 0;
    for (dh, dw) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let nh = usize::try_from(r as i32 + dh).unwrap_or(h);
        let nw = usize::try_from(c as i32 + dw).unwrap_or(w);
        if nh < h && nw < w {
            answer += 1;
        }
    }

    println!("{}", answer);
}
