use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut deg = vec![0; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }

        g.get_mut(a).unwrap().push(b);
        g.get_mut(b).unwrap().push(a);
        deg[a] += 1;
        deg[b] += 1;
    }

    if deg.into_iter().any(|x| x > 2) {
        println!("No");
        return;
    }

    let mut used = vec![false; n];
    let mut answer = true;
    for i in 0..n {
        if used[i] {
            continue;
        }

        answer &= dfs(&g, &mut used, i, n);
    }

    println!("{}", if answer { "Yes" } else { "No" });
}

fn dfs(g: &[Vec<usize>], used: &mut [bool], u: usize, p: usize) -> bool {
    let mut result = true;
    used[u] = true;
    for v in &g[u] {
        if *v == p {
            continue;
        }

        if used[*v] {
            return false;
        }

        result &= dfs(g, used, *v, u);
    }

    return result;
}
