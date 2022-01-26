use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut e1 = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1
        }
        e1[a][b] = true;
        e1[b][a] = true;
    }

    let mut e2: Vec<(usize, usize)> = Vec::with_capacity(m);
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1
        }
        e2.push((a, b));
    }

    for perm in (0..n).permutations(n) {
        let mut ok = true;
        for (a, b) in &e2 {
            ok &= e1[perm[*a]][perm[*b]];
            if !ok {
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
