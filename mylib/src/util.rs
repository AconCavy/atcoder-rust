#![allow(dead_code)]

use itertools::Itertools;
use num_integer::Integer;
use num_traits::Signed;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

pub fn compress<T: Clone + Eq + Ord + std::hash::Hash>(
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

pub fn binary_search<T: Copy + Integer + Signed, F: Fn(T) -> bool>(ng: T, ok: T, f: F) -> T {
    let mut ng = ng;
    let mut ok = ok;
    let two = T::one() + T::one();
    while (ok - ng).abs() > T::one() {
        let m = (ok + ng) / two;
        if f(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    ok
}

pub fn lower_bound<T: std::cmp::Ord>(v: &[T], key: T) -> usize {
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

pub fn upper_bound<T: std::cmp::Ord>(v: &[T], key: T) -> usize {
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

pub fn gcd<T: Copy + Integer>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T: Copy + Integer>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

pub fn ext_gcd<T: Copy + Integer>(a: T, b: T) -> (T, T, T) {
    if b == T::zero() {
        (a, T::one(), T::zero())
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

pub type Pos = (usize, usize);
pub fn d4(curr: Pos, lim: Pos) -> impl Iterator<Item = Pos> {
    let d = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut i = 0;
    std::iter::from_fn(move || {
        while i < d.len() {
            let nh = usize::try_from(curr.0 as i32 + d[i].0).unwrap_or(lim.0);
            let nw = usize::try_from(curr.1 as i32 + d[i].1).unwrap_or(lim.1);
            i += 1;
            if nh < lim.0 && nw < lim.1 {
                return Some((nh, nw));
            }
        }

        None
    })
}

#[test]
fn binary_search_test() {
    let ok = 100;
    let ng = -1;
    for v in 0..=100 {
        assert_eq!(binary_search(ng, ok, |x| x >= v), v);
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
