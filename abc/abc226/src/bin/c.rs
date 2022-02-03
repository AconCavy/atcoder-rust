use proconio::marker::Usize1;
use proconio::*;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut t: Vec<i64> = vec![0; n];
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        input! {
            ti: i64,
            k: usize
        }

        t[i] = ti;

        if k > 0 {
            input! {
                a: [Usize1; k]
            }

            for a in a {
                g[i].push(a);
            }
        }
    }

    let mut used = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(n - 1);
    while let Some(u) = queue.pop_front() {
        if used[u] {
            continue;
        }

        used[u] = true;
        for v in &g[u] {
            if !used[*v] {
                queue.push_back(*v);
            }
        }
    }

    let mut answer = 0i64;
    for (used, t) in used.into_iter().zip(t) {
        if used {
            answer += t;
        }
    }
    println!("{}", answer);
}
