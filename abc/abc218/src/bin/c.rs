use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    let mut ps = Vec::new();
    for (i, s) in s.iter().enumerate() {
        for (j, c) in s.iter().enumerate() {
            if *c == '#' {
                ps.push((i, j));
            }
        }
    }

    let mut pt = Vec::new();
    let mut tt = vec![vec![false; n * 3]; n * 3];
    for (i, t) in t.iter().enumerate() {
        for (j, c) in t.iter().enumerate() {
            if *c == '#' {
                pt.push((i, j));
                tt[n + i][n + j] = true;
            }
        }
    }

    if ps.len() != pt.len() {
        println!("No");
        return;
    }

    for _ in 0..4 {
        for si in 0..(n * 2) {
            for sj in 0..(n * 2) {
                let mut ok = true;
                for (i, j) in &ps {
                    ok &= tt[si + i][sj + j];
                    if !ok {
                        break;
                    }
                }

                if ok {
                    println!("Yes");
                    return;
                }
            }
        }

        for p in &mut ps {
            *p = (p.1, n - p.0 - 1);
        }
    }

    println!("No");
}
