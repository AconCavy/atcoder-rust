use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [i32; n],
    }

    let mut g = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }
        g[a].push(b);
        g[b].push(a);
    }

    let mut a = vec![Vec::new(); n];
    for (i, x) in x.iter().enumerate() {
        a[i].push(*x);
    }

    fn dfs(g: &[Vec<usize>], a: &mut [Vec<i32>], u: usize, p: usize) {
        let mut capacity = 1;
        for v in &g[u] {
            if *v == p {
                continue;
            }
            dfs(g, a, *v, u);
            capacity += a[*v].len();
        }

        let mut buffer = Vec::with_capacity(capacity);
        for a in &a[u] {
            buffer.push(*a);
        }

        for v in &g[u] {
            if *v == p {
                continue;
            }

            for a in &a[*v] {
                buffer.push(*a);
            }
        }

        buffer.sort();
        buffer.reverse();
        a[u] = buffer.into_iter().take(20).collect_vec();
    }

    dfs(&g, &mut a, 0, n);

    for _ in 0..q {
        input! {
            v: Usize1,
            k: Usize1,
        }

        println!("{}", a[v][k]);
    }
}
