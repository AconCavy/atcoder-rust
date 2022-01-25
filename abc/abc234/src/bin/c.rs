use proconio::*;

#[fastout]
fn main() {
    input! {
        mut k: i64,
    }

    let mut v: Vec<i64> = Vec::new();
    while k > 0 {
        v.push((k & 1) * 2);
        k >>= 1;
    }

    v.reverse();
    let answer = v.into_iter().map(|x| x.to_string()).collect::<String>();
    println!("{}", answer);
}
