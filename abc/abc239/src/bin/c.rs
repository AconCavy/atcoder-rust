use proconio::*;

#[fastout]
fn main() {
    input! {
        (x1, y1): (i32, i32),
        (x2, y2): (i32, i32),
    }

    let delta = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
        (2, -1),
        (2, 1),
    ];
    for (dx1, dy1) in &delta {
        for (dx2, dy2) in &delta {
            if x1 + dx1 == x2 + dx2 && y1 + dy1 == y2 + dy2 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
