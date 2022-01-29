use proconio::*;

#[fastout]
fn main() {
    input! {
        mut n: i32,
    }

    if n >= 42 {
        n += 1;
    }

    println!("AGC{:>03}", n);
}
