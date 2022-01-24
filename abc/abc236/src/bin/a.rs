use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        a: Usize1,
        b: Usize1,
    }

    s.swap(a, b);
    let s = s.into_iter().collect::<String>();
    println!("{}", s);
}
