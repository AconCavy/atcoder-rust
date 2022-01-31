use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut w: i64,
    }

    let mut c: Vec<(i64, i64)> = vec![(0, 0); n];
    for i in 0..n {
        input! {
            ab:(i64, i64),
        }

        c[i] = ab;
    }

    c.sort_by_key(|x| -x.0);
    let mut answer = 0i64;
    for (a, b) in c {
        let g = std::cmp::min(w, b);
        answer += a * g;
        w -= g;
    }

    println!("{}", answer);
}
