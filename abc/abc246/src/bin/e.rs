#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::io;
use std::mem::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        (ah, aw): (Usize1, Usize1),
        (bh, bw): (Usize1, Usize1),
        s: [Chars; n],
    }

    const INF: i32 = 1e9 as i32;
    let h = n;
    let w = n;
    let mut g = vec![vec![INF; w]; h];
    g[ah][aw] = 0;
    let mut used = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    queue.push_back((ah, aw));
    let d4 = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
    while let Some((ch, cw)) = queue.pop_front() {
        if used[ch][cw] {
            continue;
        }

        used[ch][cw] = true;
        let cc = g[ch][cw];
        for &(dh, dw) in &d4 {
            for d in 1..n as i32 {
                let nh = usize::try_from(ch as i32 + dh * d).unwrap_or(h);
                let nw = usize::try_from(cw as i32 + dw * d).unwrap_or(w);
                let nc = cc + 1;
                if h <= nh || w <= nw || s[nh][nw] == '#' || nc > g[nh][nw] {
                    break;
                }

                g[nh][nw] = nc;
                queue.push_back((nh, nw));
            }
        }
    }

    let answer = if g[bh][bw] == INF { -1 } else { g[bh][bw] };
    println!("{}", answer);
}
