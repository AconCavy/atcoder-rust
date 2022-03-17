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
        h: usize,
        w: usize,
        g: [Chars; h],
    }

    const INF: i32 = 1e9 as i32;
    let mut gg = vec![vec![INF; w]; h];
    gg[0][0] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0 as usize, 0 as usize));
    let d4: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((Reverse(cc), ch, cw)) = queue.pop() {
        if gg[ch][cw] < cc {
            continue;
        }

        for &(dh, dw) in &d4 {
            let (nh, nw) = (ch as i32 + dh, cw as i32 + dw);
            if nh < 0 || h as i32 <= nh || nw < 0 || w as i32 <= nw {
                continue;
            }
            let (nh, nw) = (nh as usize, nw as usize);
            if g[nh][nw] == '.' {
                if cc < gg[nh][nw] {
                    gg[nh][nw] = cc;
                    queue.push((Reverse(cc), nh, nw));
                }
            } else {
                let nc = cc + 1;
                for dh in -1..=1 {
                    for dw in -1..=1 {
                        let (nnh, nnw) = (nh as i32 + dh, nw as i32 + dw);
                        if nnh < 0 || h as i32 <= nnh || nnw < 0 || w as i32 <= nnw {
                            continue;
                        }

                        let (nnh, nnw) = (nnh as usize, nnw as usize);
                        if nc < gg[nnh][nnw] {
                            gg[nnh][nnw] = nc;
                            queue.push((Reverse(nc), nnh, nnw));
                        }
                    }
                }
            }
        }
    }

    let answer = gg[h - 1][w - 1];
    println!("{}", answer);
}
