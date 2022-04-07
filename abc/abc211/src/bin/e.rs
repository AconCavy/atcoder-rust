#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use nalgebra::DimAdd;
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
        n: usize,
        k: usize,
        mut s: [Chars; n],
    }

    let mut dp = HashSet::new();

    fn dfs(
        n: usize,
        k: usize,
        s: &mut Vec<Vec<char>>,
        depth: usize,
        dp: &mut HashSet<u64>,
        answer: &mut i32,
    ) {
        let mut state = 0;
        for i in 0..n {
            for j in 0..n {
                if s[i][j] == '@' {
                    state |= 1u64 << (i * n + j);
                }
            }
        }

        if dp.contains(&state) {
            return;
        }

        dp.insert(state);

        if depth == k {
            *answer += 1;
            return;
        }

        let d4 = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

        for ch in 0..n {
            for cw in 0..n {
                if s[ch][cw] != '.' {
                    continue;
                }

                let mut ok = depth == 0;
                for &(dh, dw) in &d4 {
                    let nh = usize::try_from(ch as i32 + dh).unwrap_or(n);
                    let nw = usize::try_from(cw as i32 + dw).unwrap_or(n);
                    if nh < n && nw < n {
                        ok |= s[nh][nw] == '@';
                    }
                }

                if ok {
                    s[ch][cw] = '@';
                    dfs(n, k, s, depth + 1, dp, answer);
                    s[ch][cw] = '.';
                }
            }
        }
    }

    let mut answer = 0;
    dfs(n, k, &mut s, 0, &mut dp, &mut answer);

    println!("{}", answer);
}
