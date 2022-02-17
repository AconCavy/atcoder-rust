use std::collections::HashSet;

use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut p = Vec::with_capacity(n);
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            (x, y): (i32, i32),
        }

        p.push((x, y));
        set.insert((x, y));
    }

    let mut answer = 0;
    for (x1, y1) in &p {
        for (x2, y2) in &p {
            if x1 == x2 || y1 == y2 {
                continue;
            }
            if set.contains(&(*x1, *y2)) && set.contains(&(*x2, *y1)) {
                answer += 1;
            }
        }
    }

    answer /= 4;
    println!("{}", answer);
}
