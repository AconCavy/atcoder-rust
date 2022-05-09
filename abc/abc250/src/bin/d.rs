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
        n: i64,
    }

    let primes = sieve(1e6 as i64);
    let m = primes.len();
    let mut answer = 0;
    let mut j = m - 1;
    for i in 0..m {
        if i >= j {
            break;
        }

        while i < j {
            let p = primes[i];
            let q3 = primes[j] * primes[j] * primes[j];
            let x = p.checked_mul(q3).unwrap_or(n + 1);
            if x > n {
                j -= 1;
            } else {
                break;
            }
        }

        answer += j - i;
    }

    println!("{}", answer);
}

pub fn sieve(v: i64) -> Vec<i64> {
    let mut result = Vec::new();
    if v < 2 {
        return result;
    }

    result.push(2);
    let mut sieve = vec![true; ((v + 1) / 2) as usize];
    for i in 1..sieve.len() {
        if sieve[i] {
            result.push(i as i64 * 2 + 1);
            let mut j = i;
            while j < sieve.len() {
                sieve[j] = false;
                j += i * 2 + 1;
            }
        }
    }

    result
}
