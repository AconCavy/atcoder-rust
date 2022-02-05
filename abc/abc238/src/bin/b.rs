use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut cut = vec![0];
    let mut curr = 0;
    for a in a {
        curr = (curr + a) % 360;
        cut.push(curr);
    }
    cut.push(360);
    cut.sort();
    let mut answer = 0;
    for i in 1..cut.len() {
        answer = std::cmp::max(answer, cut[i] - cut[i - 1]);
    }

    println!("{}", answer);
}
