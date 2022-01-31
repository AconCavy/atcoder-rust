use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s:Chars,
        k:usize,
    }

    let mut cum = vec![0; s.len() + 1];
    for (i, c) in s.iter().enumerate() {
        cum[i + 1] = cum[i];
        if *c == '.' {
            cum[i + 1] += 1;
        }
    }
    let mut l = 0;
    let mut r = 0;
    let mut answer = 0;
    while l < cum.len() {
        while r + 1 < cum.len() && cum[r + 1] - cum[l] <= k {
            r += 1;
        }

        answer = std::cmp::max(answer, r - l);
        l += 1;
    }
    println!("{}", answer);
}
