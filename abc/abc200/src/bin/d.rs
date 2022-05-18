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
        a: [usize; n],
    }

    let a = a.into_iter().map(|x| x % 200).collect::<Vec<_>>();
    let mut dp = vec![Vec::new(); 200];
    dp[a[0]].push(1);

    fn f(b: Vec<usize>, c: Vec<usize>) {
        println!("Yes");
        println!("{} {}", b.len(), b.into_iter().join(" "));
        println!("{} {}", c.len(), c.into_iter().join(" "));
    }

    for i in 1..n {
        if !dp[a[i]].is_empty() {
            let b = dp[a[i]].clone();
            let c = vec![i + 1];
            f(b, c);
            return;
        }

        let mut used = vec![false; 200];
        used[a[i]] = true;
        dp[a[i]].push(i + 1);

        for j in (0..200).rev() {
            let k = (j + a[i]) % 200;

            if !used[j] && !dp[j].is_empty() {
                if !dp[k].is_empty() {
                    let b = dp[k].clone();
                    let mut c = dp[j].clone();
                    c.push(i + 1);
                    f(b, c);
                    return;
                } else {
                    dp[k] = dp[j].clone();
                    dp[k].push(i + 1);
                    used[j] = true;
                    used[k] = true;
                }
            }
        }
    }

    println!("No");
}
