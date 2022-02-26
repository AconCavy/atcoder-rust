use proconio::*;

#[fastout]
fn main() {
    const INF: i64 = 1e18 as i64;

    input! {
        t: usize,
    }

    let f = |x: i64| x * (x + 1) / 2;
    let g = |a: i64, b: i64, x: i64, k: i64| a + b * k + x * f(k);

    for _ in 0..t {
        input! {
            n: usize,
            _: usize,
        }

        let mut a = 0i64;
        let mut b = 0i64;
        let mut answer = -INF;
        for _ in 0..n {
            input! {
                x: i64,
                y: i64,
            }

            if b > 0 && x < 0 {
                let mut k = (b / -x).min(y);
                answer = answer.max(g(a, b, x, k));
                k = (k + 1).min(y);
                answer = answer.max(g(a, b, x, k));
            }

            answer = answer.max(g(a, b, x, 1));
            answer = answer.max(g(a, b, x, y));
            a = g(a, b, x, y);
            b += x * y;
        }

        println!("{}", answer);
    }
}
