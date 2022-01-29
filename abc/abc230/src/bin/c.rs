use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i64,
        (a, b):(i64,i64),
        (p, q):(i64,i64),
        (r, s):(i64,i64),
    }

    for i in p..(q + 1) {
        for j in r..(s + 1) {
            print!(
                "{}",
                if (a - i).abs() == (b - j).abs() {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!();
    }
}
