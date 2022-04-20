#![allow(dead_code)]

use itertools::Itertools;
use num_integer::Integer;
use std::collections::{HashMap, HashSet};

fn compress<T: Clone + Eq + Ord + std::hash::Hash>(
    source: &[T],
) -> (HashMap<T, usize>, HashMap<usize, T>) {
    let set: HashSet<_> = source.iter().collect();
    let mut map: HashMap<T, usize> = HashMap::new();
    let mut remap: HashMap<usize, T> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        map.insert(x.clone(), i);
        remap.insert(i, x.clone());
    }

    (map, remap)
}

fn binary_search<F: Fn(i64) -> bool>(ng: i64, ok: i64, f: F) -> i64 {
    let mut ng = ng;
    let mut ok = ok;
    while (ok - ng).abs() > 1 {
        let m = (ok + ng) / 2;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}

fn lower_bound<T: std::cmp::Ord>(v: &[T], key: T) -> usize {
    let mut l = -1;
    let mut r = v.len() as isize;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if v[m as usize] >= key {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}

fn upper_bound<T: std::cmp::Ord>(v: &[T], key: T) -> usize {
    let mut l = -1;
    let mut r = v.len() as isize;
    while r - l > 1 {
        let m = l + (r - l) / 2;
        if v[m as usize] > key {
            r = m;
        } else {
            l = m;
        }
    }
    r as usize
}

fn gcd<T: Copy + Integer>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm<T: Copy + Integer>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

fn ext_gcd<T: Copy + Integer>(a: T, b: T) -> (T, T, T) {
    if b == T::zero() {
        (a, T::one(), T::zero())
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

#[test]
fn ext_gcd_test() {
    for a in 0..=100 {
        for b in 0..=100 {
            let (g, x, y) = ext_gcd(a, b);
            assert_eq!((a * x) + (b * y), g);
        }
    }
}
