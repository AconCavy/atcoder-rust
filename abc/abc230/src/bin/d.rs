use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i32,
        mut w: [(i32, i32);n]
    }

    w.sort_by_key(|(x, y)| (*y, *x));

    let mut rr = 0;
    let mut answer = 0;
    for (l, r) in w {
        if l <= rr {
            continue;
        }

        rr = r + d - 1;
        answer += 1;
    }

    println!("{}", answer);
}
