use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut d: Vec<u32> = Vec::new();
    for c in s {
        d.push(c.to_digit(10).unwrap());
    }

    d.sort();
    d.reverse();

    let n = d.len();
    let mut answer = 0u64;
    for i in 0..(1 << n) {
        let mut a = 0u64;
        let mut b = 0u64;
        for j in 0..n {
            if i >> j & 1 == 0 {
                a *= 10;
                a += d[j] as u64;
            } else {
                b *= 10;
                b += d[j] as u64;
            }
        }

        if a == 0 || b == 0 {
            continue;
        }

        answer = std::cmp::max(answer, a * b);
    }

    println!("{}", answer);
}
