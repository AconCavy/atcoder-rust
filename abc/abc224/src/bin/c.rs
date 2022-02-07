use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(i64, i64); n]
    }

    let mut answer = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let x1 = p[j].0 - p[i].0;
                let y1 = p[j].1 - p[i].1;
                let x2 = p[k].0 - p[i].0;
                let y2 = p[k].1 - p[i].1;
                if (x1 * y2 - x2 * y1).abs() > 0 {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer);
}
