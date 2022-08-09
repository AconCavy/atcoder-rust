#![allow(dead_code)]
#![allow(unused_imports)]

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use rand::rngs::StdRng;
use rand::Rng;
use std::cmp::*;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::io;
use std::mem::*;
use std::time::{Duration, Instant};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        g: [Bytes; n],
    }

    println!(0);

    let mut result = Vec::new();
    let mut used = g.clone();
    for h in 0..n {
        for w in 0..n {
            if g[h][w] == b'0' {
                continue;
            }

            let mut ok = true;
            for j in w + 1..n {
                if g[h][j] == g[h][w] {
                    result.push((h, w, h, j));
                    for jj in w..=j {
                        used[h][jj] = g[h][w];
                    }
                    break;
                } else {
                    ok &= g[h][j] == b'0' && used[h][j] == b'0';
                    if !ok {
                        break;
                    }
                }
            }

            if ok {
                continue;
            }

            ok = true;
            for i in h + 1..n {
                if g[i][w] == g[h][w] {
                    result.push((h, w, i, w));
                    for ii in h..=i {
                        used[ii][w] = g[h][w];
                    }
                    break;
                } else {
                    ok &= g[i][w] == b'0' && used[i][w] == b'0';
                    if !ok {
                        break;
                    }
                }
            }

            if result.len() == 100 * k {
                break;
            }
        }

        if result.len() == 100 * k {
            break;
        }
    }

    println!("{}", result.len());
    for (h0, w0, h1, w1) in result {
        println!("{} {} {} {}", h0, w0, h1, w1);
    }
}
