use proconio::*;

#[fastout]
fn main() {
    input! {
        n: i32,
        h: [i32; n],
    }

    let mut answer = 0;
    for x in h {
        if answer < x {
            answer = x;
        } else {
            break;
        }
    }

    println!("{}", answer);
}
