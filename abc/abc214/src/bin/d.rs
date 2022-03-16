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
    }

    let mut edges = Vec::with_capacity(n - 1);
    for _ in 0..(n - 1) {
        input! {
            u: Usize1,
            v: Usize1,
            w: i64,
        }
        edges.push((u, v, w));
    }

    edges.sort_by_key(|e| e.2);
    let mut answer = 0;
    let mut dsu = Dsu::new(n);
    for (u, v, w) in edges {
        if !dsu.same(u, v) {
            answer += w * dsu.size_of(u) as i64 * dsu.size_of(v) as i64;
            dsu.merge(u, v);
        }
    }

    println!("{}", answer);
}

struct Dsu {
    len: usize,
    parent_or_size: Vec<i32>,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            len: size,
            parent_or_size: vec![-1; size],
        }
    }

    fn merge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.len);
        assert!(v < self.len);
        let (mut x, mut y) = (self.leader_of(u), self.leader_of(v));
        if x == y {
            return x;
        }

        if -self.parent_or_size[u] < -self.parent_or_size[v] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        assert!(u < self.len);
        assert!(v < self.len);
        self.leader_of(u) == self.leader_of(v)
    }

    fn leader_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        if self.parent_or_size[u] < 0 {
            return u;
        }

        self.parent_or_size[u] = self.leader_of(self.parent_or_size[u] as usize) as i32;
        self.parent_or_size[u] as usize
    }

    fn size_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        let u = self.leader_of(u);
        -self.parent_or_size[u] as usize
    }

    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_of = vec![0; self.len];
        let mut group_size = vec![0; self.len];
        for i in 0..self.len {
            leader_of[i] = self.leader_of(i);
            group_size[leader_of[i]] += 1;
        }
        let mut result: Vec<Vec<_>> = group_size
            .into_iter()
            .map(|c| Vec::with_capacity(c))
            .collect();
        for (i, v) in leader_of.into_iter().enumerate() {
            result[v].push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
