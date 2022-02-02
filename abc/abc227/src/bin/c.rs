use proconio::*;

#[fastout]
fn main() {
    input! {
        n:i64,
    }

    let mut answer = 0i64;
    let mut a = 1i64;
    loop {
        if a * a * a > n {
            break;
        }

        let mut b = a;
        loop {
            if a * b * b > n {
                break;
            }

            let c = n / a / b;
            if c < b {
                break;
            }

            answer += c - b + 1;
            b += 1;
        }

        a += 1;
    }

    println!("{}", answer);
}
