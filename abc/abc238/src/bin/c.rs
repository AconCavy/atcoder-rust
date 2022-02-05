use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i64
    }

    let m = 998244353;
    let mut answer = 0i64;
    let f = |x: i64, m: i64| -> i64 { ((x % m) * ((x + 1) % m) % m * inv(2, m)) % m };
    let mut curr = 1i64;
    loop {
        curr *= 10;
        let x = if curr <= n {
            curr - curr / 10
        } else {
            n - curr / 10 + 1
        };
        answer = (answer + f(x, m)) % m;

        if curr > n {
            break;
        }
    }

    println!("{}", answer);
}

fn inv(v: i64, m: i64) -> i64 {
    pow(v, m - 2, m)
}

fn pow(v: i64, n: i64, m: i64) -> i64 {
    assert!(m >= 0);
    let mut v = v;
    let mut n = n;
    let mut result = 1i64;
    while n > 0 {
        if n & 1 > 0 {
            result = result * v % m;
        }

        v = v * v % m;
        n >>= 1;
    }
    result
}
