use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }

    let mut t = ab.iter().map(|x| x.0 / x.1).sum::<f64>() / 2.0;
    let mut answer = 0f64;
    for (a, b) in ab {
        let x = t.min(a / b);
        t -= x;
        answer += x * b;
    }

    println!("{}", answer);
}
