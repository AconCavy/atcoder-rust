#![allow(dead_code)]

use std::collections::{BinaryHeap, VecDeque};

use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut d: [usize; n],
    }

    if d.iter().fold(0, |acc, x| acc + x) != (n - 1) * 2 {
        println!("-1");
        return;
    }

    let mut g = vec![Vec::new(); n];
    let mut dsu = Dsu::new(n);

    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }

        dsu.merge(a, b);
        g[a].push(b);
        g[b].push(a);
        d[a] -= 1;
        d[b] -= 1;
    }

    let mut heap = BinaryHeap::new();
    for group in dsu.groups() {
        let sum = group.iter().fold(0, |acc, &x| acc + d[x]);
        let queue: VecDeque<_> = group.into_iter().filter(|&x| d[x] > 0).collect();
        if queue.len() == 0 {
            println!("-1");
            return;
        }
        heap.push((sum, queue));
    }

    let mut answer = Vec::with_capacity(n - m - 1);
    while heap.len() > 1 {
        let (us, mut uq) = heap.pop().unwrap();
        let (vs, mut vq) = heap.pop().unwrap();
        let u = uq.pop_front().unwrap();
        let v = vq.pop_front().unwrap();
        answer.push((u + 1, v + 1));
        d[u] -= 1;
        d[v] -= 1;
        if d[u] > 0 {
            uq.push_back(u);
        }
        if d[v] > 0 {
            vq.push_back(v);
        }
        if us >= vs {
            while let Some(x) = vq.pop_front() {
                uq.push_back(x);
            }

            heap.push((us + vs - 2, uq));
        } else {
            while let Some(x) = uq.pop_front() {
                vq.push_back(x);
            }

            heap.push((us + vs - 2, vq));
        }
    }

    if let Some((_, mut queue)) = heap.pop() {
        while queue.len() > 1 {
            let u = queue.pop_front().unwrap();
            let v = queue.pop_front().unwrap();
            answer.push((u + 1, v + 1));
            d[u] -= 1;
            d[v] -= 1;
            if d[u] > 0 {
                queue.push_back(u);
            }
            if d[v] > 0 {
                queue.push_back(v);
            }
        }
    }

    if d.into_iter().any(|x| x != 0) {
        println!("-1");
    } else {
        for (u, v) in answer {
            println!("{} {}", u, v);
        }
    }
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
        let mut result: Vec<Vec<usize>> = vec![Vec::new(); self.len];
        for i in 0..self.len {
            result.get_mut(self.leader_of(i)).unwrap().push(i);
        }
        result.into_iter().filter(|x| !x.is_empty()).collect()
    }
}
