use proconio::*;

#[fastout]
fn main() {
    input! {
        t: i32
    }

    for _ in 0..t {
        input! {
            a: i64,
            s: i64
        }

        let a2 = a + a;
        let answer = a2 <= s && (s - a2) & a == 0;
        println!("{}", if answer { "Yes" } else { "No" });
    }
}
