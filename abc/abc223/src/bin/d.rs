use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize
    }

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut deg = vec![0; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1
        }

        g[a].push(b);
        deg[b] += 1;
    }

    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    let mut answer: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        if deg[i] == 0 {
            heap.push(n - i);
        }
    }

    while let Some(u) = heap.pop() {
        let u = n - u;
        answer.push(u + 1);
        for &v in &g[u] {
            deg[v] -= 1;
            if deg[v] == 0 {
                heap.push(n - v);
            }
        }
    }

    let answer = if answer.len() == n {
        answer.into_iter().join(" ")
    } else {
        "-1".to_string()
    };
    println!("{}", answer);
}
