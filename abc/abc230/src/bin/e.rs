use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    let mut answer = 0i64;
    let mut i = 1i64;
    loop {
        if i * i > n {
            break;
        }

        answer += n / i;
        if n / i != i {
            answer += (n / i - n / (i + 1)) * i;
        }

        i += 1;
    }

    println!("{}", answer);
}
