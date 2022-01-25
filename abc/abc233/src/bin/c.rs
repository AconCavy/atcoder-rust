use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
    }

    let mut answer: i64 = 0;
    let mut a: Vec<Vec<i64>> = Vec::new();
    for _ in 0..n {
        input! {
            l: usize,
            b: [i64; l],
        }

        a.push(b.into_iter().filter(|v| x % v == 0).collect());
    }

    dfs(&mut answer, &a, 0, n, x);
    println!("{}", answer)
}

fn dfs(answer: &mut i64, a: &[Vec<i64>], idx: usize, end: usize, x: i64) {
    if idx >= end {
        if x == 1 {
            *answer += 1;
        }
        return;
    }

    for y in &a[idx] {
        if x % y == 0 {
            dfs(answer, a, idx + 1, end, x / y);
        }
    }
}
