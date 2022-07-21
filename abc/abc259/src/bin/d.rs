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
        (sx, sy): (i64, i64),
        (tx, ty): (i64, i64),
    }

    let s = Circle { x: sx, y: sy, r: 0 };
    let t = Circle { x: tx, y: ty, r: 0 };

    let mut circles = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            x: i64,
            y: i64,
            r: i64,
        }

        circles.push(Circle { x, y, r });
    }

    fn square_distance(c0: &Circle, c1: &Circle) -> i64 {
        let dx = c0.x - c1.x;
        let dy = c0.y - c1.y;
        dx * dx + dy * dy
    }

    fn intersect(c0: &Circle, c1: &Circle) -> bool {
        let sqd = square_distance(c0, c1);
        let add = c0.r + c1.r;
        let sub = c0.r - c1.r;
        sub * sub <= sqd && sqd <= add * add
    }

    let mut dsu = Dsu::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if intersect(&circles[i], &circles[j]) {
                dsu.merge(i, j);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if intersect(&s, &circles[i]) && intersect(&t, &circles[j]) && dsu.same(i, j) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

pub struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

pub struct Dsu {
    len: usize,
    parent_or_size: Vec<i32>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            len: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, u: usize, v: usize) -> usize {
        assert!(u < self.len);
        assert!(v < self.len);
        let (mut x, mut y) = (self.leader_of(u), self.leader_of(v));
        if x == y {
            return x;
        }

        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        assert!(u < self.len);
        assert!(v < self.len);
        self.leader_of(u) == self.leader_of(v)
    }

    pub fn leader_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        if self.parent_or_size[u] < 0 {
            return u;
        }

        self.parent_or_size[u] = self.leader_of(self.parent_or_size[u] as usize) as i32;
        self.parent_or_size[u] as usize
    }

    pub fn size_of(&mut self, u: usize) -> usize {
        assert!(u < self.len);
        let u = self.leader_of(u);
        -self.parent_or_size[u] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut leader_of = vec![0; self.len];
        let mut group_size = vec![0; self.len];
        for i in 0..self.len {
            leader_of[i] = self.leader_of(i);
            group_size[leader_of[i]] += 1;
        }
        let mut result: Vec<Vec<_>> = group_size.into_iter().map(Vec::with_capacity).collect();
        for (i, v) in leader_of.into_iter().enumerate() {
            result[v].push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
