use proconio::marker::Usize1;
use proconio::*;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        h:[i64; n],
    }

    let mut g: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }

        g[u].push((v, std::cmp::max(0, h[v] - h[u])));
        g[v].push((u, std::cmp::max(0, h[u] - h[v])));
    }

    let mut cost = vec![1e18 as i64; n];
    cost[0] = 0;
    let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
    heap.push((-0, 0));
    while let Some((uc, u)) = heap.pop() {
        let uc = -uc;
        if cost[u] < uc {
            continue;
        }

        for (v, vc) in &g[u] {
            let c = cost[u] + *vc;
            if c < cost[*v] {
                cost[*v] = c;
                heap.push((-c, *v));
            }
        }
    }

    let answer = (0..n).map(|i| h[0] - h[i] - cost[i]).max().unwrap();
    println!("{}", answer);
}
