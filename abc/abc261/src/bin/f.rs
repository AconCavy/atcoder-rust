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
        c: [usize; n],
        x: [usize; n],
    }

    let mut answer = inversion_number(&x);

    let mut map = HashMap::new();
    for (&c, &x) in c.iter().zip(x.iter()) {
        map.entry(c).or_insert_with(Vec::new).push(x);
    }
    for (_, vals) in map.into_iter() {
        answer -= inversion_number(&vals);
    }

    println!("{}", answer);
}

pub fn inversion_number(vals: &[usize]) -> usize {
    let mut result = 0;
    let (map, _) = compress(vals);
    let mut bit = FenwickTree::new(vals.len() + 1, 0);
    for i in 0..vals.len() {
        let v = *map.get(&vals[i]).unwrap();
        result += i - bit.accum(v + 1);
        bit.add(v, 1);
    }

    result
}

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

pub struct FenwickTree<T> {
    len: usize,
    data: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign> FenwickTree<T> {
    pub fn new(len: usize, e: T) -> Self {
        Self {
            len,
            data: vec![e.clone(); len],
            e,
        }
    }

    pub fn add(&mut self, i: usize, v: T) {
        assert!(i < self.len);
        let mut i = i as i32 + 1;
        while i as usize <= self.len {
            self.data[(i - 1) as usize] += v.clone();
            i += i & -i;
        }
    }

    pub fn accum(&self, len: usize) -> T {
        assert!(len <= self.len);
        let mut len = len as i32;
        let mut sum = self.e.clone();
        while len > 0 {
            sum += self.data[(len - 1) as usize].clone();
            len -= len & -len;
        }

        sum
    }

    pub fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        assert!(l <= r && r <= self.len);
        self.accum(r) - self.accum(l)
    }

    pub fn bound<F: Fn(T, T) -> bool>(&self, v: T, compare: F) -> usize
    where
        T: std::ops::SubAssign,
    {
        if compare(v.clone(), self.data[0].clone()) {
            return 0;
        }

        let mut v = v.clone();
        let mut x = 0;
        let mut k = 1;
        while k < self.len {
            k <<= 1;
        }

        while k > 0 {
            if x + k > self.len || compare(v.clone(), self.data[x + k - 1].clone()) {
                k >>= 1;
                continue;
            }

            v -= self.data[x + k - 1].clone();
            x += k;
            k >>= 1;
        }

        x
    }
}
