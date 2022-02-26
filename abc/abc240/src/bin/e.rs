use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    const INF: i32 = 1e9 as i32;

    let mut g = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        input! {
            u: Usize1,
            v: Usize1,
        }

        g[u].push(v);
        g[v].push(u);
    }

    let mut l = vec![0; n];
    let mut r = vec![0; n];
    let mut curr = 1;

    fn dfs(g: &[Vec<usize>], l: &mut [i32], r: &mut [i32], curr: &mut i32, u: usize, p: usize) {
        let mut is_leaf = true;
        let mut ll = INF;
        let mut rr = -INF;
        for &v in &g[u] {
            if v == p {
                continue;
            }

            dfs(g, l, r, curr, v, u);
            is_leaf = false;
            ll = ll.min(l[v]);
            rr = rr.max(r[v]);
        }

        if is_leaf {
            ll = *curr;
            rr = *curr;
            *curr += 1;
        }

        l[u] = ll;
        r[u] = rr;
    }

    dfs(&g, &mut l, &mut r, &mut curr, 0, n);
    for (l, r) in l.into_iter().zip(r.into_iter()) {
        println!("{} {}", l, r);
    }
}
