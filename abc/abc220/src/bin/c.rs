use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        x: i64
    }

    let mut cum = vec![0; n + 1];
    for (i, a) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + a;
    }

    let y = x / cum[n];
    let z = y * cum[n];
    for (i, s) in cum.iter().enumerate() {
        if z + s <= x {
            continue;
        }

        let answer = y * n as i64 + i as i64;
        println!("{}", answer);
        return;
    }
}
