use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        a:Usize1,
    }

    let mut answer = (a + k) % n;
    if answer == 0 {
        answer = n;
    }
    println!("{}", answer);
}
